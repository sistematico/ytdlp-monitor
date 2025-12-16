#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::process::Stdio;
use std::collections::{VecDeque, HashSet};
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Manager,
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, MouseButton, MouseButtonState},
};
use tauri_plugin_notification::NotificationExt;
use chrono::{DateTime, Utc};

#[cfg(target_os = "windows")]
use clipboard_win::{formats, get_clipboard};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// Estruturas de configuração
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    download_path: String,
    filename_template: String,
    max_concurrent: usize,
    enable_notifications: bool,
    enable_sound: bool,
    check_duplicates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            download_path: "C:\\Users\\Lucas\\Desktop".to_string(),
            filename_template: "%(title)s.%(ext)s".to_string(),
            max_concurrent: 3,
            enable_notifications: true,
            enable_sound: true,
            check_duplicates: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DownloadHistory {
    urls: HashSet<String>,
    last_updated: DateTime<Utc>,
}

impl DownloadHistory {
    fn new() -> Self {
        Self {
            urls: HashSet::new(),
            last_updated: Utc::now(),
        }
    }

    fn add(&mut self, url: String) {
        self.urls.insert(url);
        self.last_updated = Utc::now();
    }

    fn contains(&self, url: &str) -> bool {
        self.urls.contains(url)
    }
}

#[derive(Debug, Clone, Serialize)]
struct DownloadItem {
    url: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct DownloadStatus {
    active: usize,
    queued: usize,
    downloads: Vec<DownloadItem>,
}

// Gerenciador de configurações
struct SettingsManager {
    settings: Arc<Mutex<Settings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ytdl-monitor");
        
        fs::create_dir_all(&config_dir).ok();
        
        let config_path = config_dir.join("settings.json");
        let settings = Self::load_from_file(&config_path);
        
        Self {
            settings: Arc::new(Mutex::new(settings)),
            config_path,
        }
    }

    fn load_from_file(path: &PathBuf) -> Settings {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self) {
        let settings = self.settings.lock().unwrap().clone();
        if let Ok(json) = serde_json::to_string_pretty(&settings) {
            fs::write(&self.config_path, json).ok();
        }
    }

    fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    fn update(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;
        self.save();
    }
}

// Gerenciador de histórico
struct HistoryManager {
    history: Arc<Mutex<DownloadHistory>>,
    history_path: PathBuf,
}

impl HistoryManager {
    fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ytdl-monitor");
        
        let history_path = config_dir.join("history.json");
        let history = Self::load_from_file(&history_path);
        
        Self {
            history: Arc::new(Mutex::new(history)),
            history_path,
        }
    }

    fn load_from_file(path: &PathBuf) -> DownloadHistory {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(DownloadHistory::new)
    }

    fn save(&self) {
        let history = self.history.lock().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*history) {
            fs::write(&self.history_path, json).ok();
        }
    }

    fn add(&self, url: String) {
        self.history.lock().unwrap().add(url);
        self.save();
    }

    fn contains(&self, url: &str) -> bool {
        self.history.lock().unwrap().contains(url)
    }
}

// Fila de downloads
struct DownloadQueue {
    queue: Arc<Mutex<VecDeque<String>>>,
    active: Arc<Mutex<Vec<String>>>,
}

impl DownloadQueue {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            active: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add(&self, url: String) -> bool {
        let mut queue = self.queue.lock().unwrap();
        let active = self.active.lock().unwrap();
        
        if queue.contains(&url) || active.contains(&url) {
            return false;
        }
        
        queue.push_back(url);
        true
    }

    fn get_next(&self) -> Option<String> {
        self.queue.lock().unwrap().pop_front()
    }

    fn can_start(&self, max_concurrent: usize) -> bool {
        self.active.lock().unwrap().len() < max_concurrent
    }

    fn mark_active(&self, url: String) {
        self.active.lock().unwrap().push(url);
    }

    fn remove_active(&self, url: &str) {
        self.active.lock().unwrap().retain(|u| u != url);
    }

    fn get_status(&self) -> DownloadStatus {
        let active = self.active.lock().unwrap();
        let queue = self.queue.lock().unwrap();
        
        let mut downloads = Vec::new();
        
        for url in active.iter() {
            downloads.push(DownloadItem {
                url: url.clone(),
                status: "active".to_string(),
            });
        }
        
        for url in queue.iter() {
            downloads.push(DownloadItem {
                url: url.clone(),
                status: "queued".to_string(),
            });
        }
        
        DownloadStatus {
            active: active.len(),
            queued: queue.len(),
            downloads,
        }
    }
}

// Função para obter o conteúdo da área de transferência
#[cfg(target_os = "windows")]
fn get_clipboard_text() -> Option<String> {
    match get_clipboard::<String, _>(formats::Unicode) {
        Ok(text) => Some(text),
        Err(_) => None,
    }
}

// Função para verificar se é uma URL de vídeo válida
fn is_video_url(url: &str) -> bool {
    let video_domains = [
        "pornhub.com",
        "xvideos.com",
        "xhamster.com",
        "youtube.com",
        "youtu.be",
        "vimeo.com",
        "dailymotion.com",
        "twitch.tv",
        "tiktok.com",
        "instagram.com",
        "facebook.com",
        "twitter.com",
        "x.com",
    ];

    let url_lower = url.to_lowercase();
    
    if !url_lower.starts_with("http://") && !url_lower.starts_with("https://") {
        return false;
    }

    video_domains.iter().any(|domain| url_lower.contains(domain))
}

// Função para tocar som de conclusão
fn play_completion_sound() {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let _ = Command::new("powershell")
            .args(&[
                "-c",
                "(New-Object Media.SoundPlayer 'C:\\Windows\\Media\\Windows Notify System Generic.wav').PlaySync()"
            ])
            .creation_flags(0x08000000)
            .output();
    }
}

// Função para baixar o vídeo usando yt-dlp
fn download_video(
    url: &str,
    app_handle: &AppHandle,
    settings: &Settings,
    history: Arc<HistoryManager>,
) -> bool {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("yt-dlp")
        .arg("-f")
        .arg("best")
        .arg("-o")
        .arg(format!("{}\\{}", settings.download_path, settings.filename_template))
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .output();

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new("yt-dlp")
        .arg("-f")
        .arg("best")
        .arg("-o")
        .arg(format!("{}/{}", settings.download_path, settings.filename_template))
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                history.add(url.to_string());
                
                if settings.enable_notifications {
                    let _ = app_handle.notification()
                        .builder()
                        .title("Download Concluído")
                        .body("Vídeo baixado com sucesso!")
                        .show();
                }
                
                if settings.enable_sound {
                    play_completion_sound();
                }
                
                true
            } else {
                if settings.enable_notifications {
                    let _ = app_handle.notification()
                        .builder()
                        .title("Erro no Download")
                        .body("Não foi possível baixar o vídeo")
                        .show();
                }
                false
            }
        }
        Err(_e) => {
            if settings.enable_notifications {
                let _ = app_handle.notification()
                    .builder()
                    .title("Erro")
                    .body("yt-dlp não encontrado. Verifique a instalação.")
                    .show();
            }
            false
        }
    }
}

// Processador de fila de downloads
fn process_download_queue(
    queue: Arc<DownloadQueue>,
    app_handle: AppHandle,
    settings_manager: Arc<SettingsManager>,
    history_manager: Arc<HistoryManager>,
) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));

            let settings = settings_manager.get();
            
            if queue.can_start(settings.max_concurrent) {
                if let Some(url) = queue.get_next() {
                    let queue_clone = queue.clone();
                    let app_clone = app_handle.clone();
                    let settings_clone = settings.clone();
                    let history_clone = history_manager.clone();
                    let url_clone = url.clone();
                    
                    queue.mark_active(url.clone());
                    
                    if settings.enable_notifications {
                        let status = queue.get_status();
                        let _ = app_handle.notification()
                            .builder()
                            .title("Download Iniciado")
                            .body(format!(
                                "{} ativo{}, {} na fila",
                                status.active,
                                if status.active > 1 { "s" } else { "" },
                                status.queued
                            ))
                            .show();
                    }
                    
                    thread::spawn(move || {
                        download_video(&url_clone, &app_clone, &settings_clone, history_clone);
                        queue_clone.remove_active(&url_clone);
                    });
                }
            }
        }
    });
}

// Comandos Tauri
#[tauri::command]
fn get_settings(settings_manager: tauri::State<Arc<SettingsManager>>) -> Settings {
    settings_manager.get()
}

#[tauri::command]
fn save_settings(
    settings: Settings,
    settings_manager: tauri::State<Arc<SettingsManager>>,
) {
    settings_manager.update(settings);
}

#[tauri::command]
fn get_download_status(queue: tauri::State<Arc<DownloadQueue>>) -> DownloadStatus {
    queue.get_status()
}

// Comandos para abrir janelas
#[tauri::command]
fn open_settings_window(app: AppHandle) {
    let window_label = "settings";
    
    // Se janela já existe, apenas mostra
    if let Some(window) = app.get_webview_window(window_label) {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    
    // Criar nova janela
    use tauri::WebviewWindowBuilder;
    use tauri::WebviewUrl;
    
    let _window = WebviewWindowBuilder::new(
        &app,
        window_label,
        WebviewUrl::App("settings.html".into())
    )
    .title("Configurações - YT-DLP Monitor")
    .inner_size(600.0, 700.0)
    .resizable(true)
    .center()
    .build();
}

#[tauri::command]
fn open_progress_window(app: AppHandle) {
    let window_label = "progress";
    
    // Se janela já existe, apenas mostra
    if let Some(window) = app.get_webview_window(window_label) {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    
    // Criar nova janela
    use tauri::WebviewWindowBuilder;
    use tauri::WebviewUrl;
    
    let _window = WebviewWindowBuilder::new(
        &app,
        window_label,
        WebviewUrl::App("progress.html".into())
    )
    .title("Downloads - YT-DLP Monitor")
    .inner_size(500.0, 400.0)
    .resizable(true)
    .center()
    .build();
}

fn main() {
    let settings_manager = Arc::new(SettingsManager::new());
    let history_manager = Arc::new(HistoryManager::new());
    let download_queue = Arc::new(DownloadQueue::new());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(settings_manager.clone())
        .manage(history_manager.clone())
        .manage(download_queue.clone())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_download_status,
            open_settings_window,
            open_progress_window,
        ])
        .setup(move |app| {
            let last_url = Arc::new(Mutex::new(String::new()));
            
            // Criar menu do tray
            let progress_item = MenuItem::with_id(app, "progress", "📊 Ver Progresso", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "⚙️ Configurações", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "❌ Sair", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[
                &progress_item,
                &settings_item,
                &quit_item,
            ])?;

            let app_handle = app.handle().clone();
            
            // Iniciar processador de fila
            process_download_queue(
                download_queue.clone(),
                app_handle.clone(),
                settings_manager.clone(),
                history_manager.clone(),
            );
            
            // Criar tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("YT-DLP Monitor - Clique para baixar")
                .on_menu_event({
                    //let app_handle = app_handle.clone();
                    
                    move |app_local, event| {
                        match event.id.as_ref() {
                            "progress" => {
                                open_progress_window(app_local.clone());
                            }
                            "settings" => {
                                open_settings_window(app_local.clone());
                            }
                            "quit" => {
                                std::process::exit(0);
                            }
                            _ => {}
                        }
                    }
                })
                // .on_menu_event(move |_app, event| {
                //     match event.id.as_ref() {
                //         "quit" => {
                //             std::process::exit(0);
                //         }
                //         _ => {}
                //     }
                // })
                .on_tray_icon_event({
                    let last_url = last_url.clone();
                    let download_queue = download_queue.clone();
                    let settings_manager = settings_manager.clone();
                    let history_manager = history_manager.clone();
                    let app_handle = app_handle.clone();
                    
                    move |_tray, event| {
                        if let tauri::tray::TrayIconEvent::Click { 
                            button: MouseButton::Left, 
                            button_state: MouseButtonState::Up, 
                            .. 
                        } = event {
                            let url = last_url.lock().unwrap().clone();
                            let settings = settings_manager.get();
                            
                            if url.is_empty() {
                                if settings.enable_notifications {
                                    let _ = app_handle.notification()
                                        .builder()
                                        .title("Sem URL")
                                        .body("Copie uma URL de vídeo primeiro")
                                        .show();
                                }
                                return;
                            }
                            
                            if settings.check_duplicates && history_manager.contains(&url) {
                                if settings.enable_notifications {
                                    let _ = app_handle.notification()
                                        .builder()
                                        .title("Vídeo Já Baixado")
                                        .body("Este vídeo já foi baixado anteriormente")
                                        .show();
                                }
                                return;
                            }
                            
                            if download_queue.add(url.clone()) {
                                let status = download_queue.get_status();
                                
                                if settings.enable_notifications {
                                    let _ = app_handle.notification()
                                        .builder()
                                        .title("Adicionado à Fila")
                                        .body(format!(
                                            "{} ativo{}, {} na fila",
                                            status.active,
                                            if status.active > 1 { "s" } else { "" },
                                            status.queued
                                        ))
                                        .show();
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            // Thread para monitorar a área de transferência
            let monitor_last_url = last_url.clone();
            
            thread::spawn(move || {
                let mut previous_clipboard = String::new();

                loop {
                    thread::sleep(Duration::from_millis(500));

                    #[cfg(target_os = "windows")]
                    if let Some(clipboard_content) = get_clipboard_text() {
                        if clipboard_content != previous_clipboard {
                            previous_clipboard = clipboard_content.clone();

                            if is_video_url(&clipboard_content) {
                                *monitor_last_url.lock().unwrap() = clipboard_content.clone();
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao executar aplicação Tauri");
}