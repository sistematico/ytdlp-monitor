# Configurações Avançadas (Versão Futura)

Este arquivo documenta possíveis melhorias e configurações que podem ser adicionadas ao projeto.

## Arquivo de Configuração (config.json)

```json
{
  "download_path": "C:\\Users\\Lucas\\Desktop",
  "auto_download": false,
  "quality": "best",
  "format": "mp4",
  "notifications_enabled": true,
  "monitored_sites": [
    "youtube.com",
    "youtu.be",
    "vimeo.com",
    "tiktok.com"
  ],
  "max_concurrent_downloads": 1,
  "download_history_limit": 50
}
```

## Funcionalidades Futuras

### 1. Interface de Configuração

```rust
// Adicionar janela de configurações
#[tauri::command]
fn open_settings(app_handle: AppHandle) {
    // Criar janela de configurações
}

#[tauri::command]
fn save_settings(config: Config) -> Result<(), String> {
    // Salvar configurações
}
```

### 2. Histórico de Downloads

```rust
struct DownloadHistory {
    url: String,
    title: String,
    timestamp: DateTime<Utc>,
    status: DownloadStatus,
    file_path: String,
}
```

### 3. Opções de Qualidade

```rust
enum Quality {
    Best,
    High1080,
    Medium720,
    Low480,
    AudioOnly,
}
```

### 4. Download Automático

```rust
// Adicionar toggle para download automático
if config.auto_download && is_video_url(&url) {
    download_video(&url, app_handle.clone());
}
```

### 5. Suporte a Playlists

```rust
fn detect_playlist(url: &str) -> bool {
    url.contains("list=") || url.contains("playlist")
}

fn download_playlist(url: &str, app_handle: AppHandle) {
    // Baixar todos os vídeos da playlist
}
```

### 6. Fila de Downloads

```rust
struct DownloadQueue {
    pending: Vec<DownloadItem>,
    current: Option<DownloadItem>,
    completed: Vec<DownloadItem>,
}
```

### 7. Progresso de Download

```rust
// Usar yt-dlp --progress para mostrar progresso
fn download_with_progress(url: &str, callback: impl Fn(f32)) {
    // Executar yt-dlp e parsear progresso
}
```

### 8. Integração com Navegadores

```rust
// Extensão de navegador para enviar URLs diretamente
// Sem necessidade de copiar para área de transferência
```

### 9. Atalhos de Teclado

```rust
// Adicionar atalhos globais do Windows
// Ex: Ctrl+Shift+D para download rápido
```

### 10. Temas do Systray

```rust
// Detectar tema do Windows (claro/escuro)
// Usar ícone apropriado
fn get_system_theme() -> Theme {
    // Detectar tema
}
```

## Estrutura de Dados para Configuração

```rust
#[derive(Serialize, Deserialize)]
struct AppConfig {
    download_settings: DownloadSettings,
    ui_settings: UISettings,
    advanced_settings: AdvancedSettings,
}

#[derive(Serialize, Deserialize)]
struct DownloadSettings {
    path: String,
    quality: Quality,
    format: String,
    auto_download: bool,
}

#[derive(Serialize, Deserialize)]
struct UISettings {
    notifications: bool,
    theme: Theme,
    language: String,
}

#[derive(Serialize, Deserialize)]
struct AdvancedSettings {
    max_concurrent: u8,
    retry_attempts: u8,
    timeout_seconds: u32,
}
```

## Melhorias de UX

### Menu do Systray Expandido

```rust
let tray_menu = SystemTrayMenu::new()
    .add_item(status_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(download_item)
    .add_item(pause_item)
    .add_item(settings_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(history_item)
    .add_item(open_folder_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(about_item)
    .add_item(quit_item);
```

### Notificações Ricas

```rust
// Notificação com botões de ação
Notification::new()
    .title("Download Concluído")
    .body(&format!("Vídeo: {}", title))
    .action("Abrir Pasta", || {
        // Abrir pasta de downloads
    })
    .action("Reproduzir", || {
        // Abrir vídeo no player padrão
    })
    .show();
```

## Integração com Serviços

### Cloud Storage

```rust
// Upload automático para Google Drive, OneDrive, etc.
async fn upload_to_cloud(file_path: &str, service: CloudService) {
    // Upload
}
```

### Conversão de Formato

```rust
// Usar FFmpeg para converter após download
fn convert_video(input: &str, output_format: &str) {
    // Conversão
}
```

## Logging e Debugging

```rust
use log::{info, warn, error};

fn setup_logger() {
    // Configurar arquivo de log
    // C:\Users\Lucas\AppData\Local\ytdl-monitor\logs\
}
```

## Atualizações Automáticas

```rust
// Usar tauri-plugin-updater
use tauri_plugin_updater::UpdaterBuilder;

fn check_for_updates(app: &AppHandle) {
    // Verificar atualizações
}
```

---

**Nota:** Estas são sugestões para versões futuras. A versão atual (v0.1.0) 
foca na funcionalidade core: monitorar área de transferência e baixar vídeos.