# YT-DLP Monitor - Monitor de URLs para Download de Vídeos

Aplicativo para Windows 11 que monitora URLs copiadas na área de transferência e baixa vídeos automaticamente usando yt-dlp.

## Funcionalidades

✅ Roda no systray do Windows  
✅ Monitora a área de transferência automaticamente  
✅ Detecta URLs de vídeo de várias plataformas (YouTube, Vimeo, TikTok, etc.)  
✅ Baixa vídeos com um clique no ícone do systray  
✅ Notificações para feedback visual  
✅ Downloads salvos automaticamente na área de trabalho  

## Pré-requisitos

1. **yt-dlp instalado e no PATH** (já configurado conforme requisito)
2. **Rust e Cargo** - Instale de: https://www.rust-lang.org/tools/install
3. **Tauri CLI** - Instale com:
   ```bash
   cargo install tauri-cli
   ```

## Como Compilar

1. Navegue até a pasta do projeto:
   ```bash
   cd ytdl-monitor
   ```

2. Compile o projeto:
   ```bash
   cargo tauri build
   ```

3. O executável estará em:
   ```
   ytdl-monitor/src-tauri/target/release/ytdl-monitor.exe
   ```

## Como Usar

1. Execute o `ytdl-monitor.exe`
2. O aplicativo aparecerá no systray (área de notificação)
3. Copie uma URL de vídeo (YouTube, TikTok, etc.)
4. O aplicativo detectará automaticamente e mostrará uma notificação
5. **Clique com o botão esquerdo** no ícone do systray para iniciar o download
6. O vídeo será baixado para: `C:\Users\Lucas\Desktop`

## Plataformas Suportadas

- YouTube (youtube.com, youtu.be)
- Vimeo
- Dailymotion
- Twitch
- TikTok
- Instagram
- Facebook
- Twitter/X

## Menus do Systray

- **Aguardando URL...** - Mostra o status atual
- **Baixar Último Vídeo** - Baixa manualmente a última URL detectada
- **Sair** - Fecha o aplicativo

## Estrutura do Projeto

```
ytdl-monitor/
├── src/                    # Frontend (HTML básico)
│   └── index.html
├── src-tauri/              # Backend Rust
│   ├── src/
│   │   └── main.rs        # Lógica principal
│   ├── Cargo.toml         # Dependências Rust
│   ├── tauri.conf.json    # Configuração do Tauri
│   └── build.rs           # Script de build
└── README.md
```

## Desenvolvimento

Para executar em modo de desenvolvimento:

```bash
cargo tauri dev
```

## Notas Técnicas

### Monitoramento da Área de Transferência
- Verifica a área de transferência a cada 500ms
- Usa a biblioteca `clipboard-win` para acesso nativo no Windows
- Detecta apenas URLs que começam com http:// ou https://

### Download de Vídeos
- Usa o comando `yt-dlp` diretamente via `std::process::Command`
- Formato padrão: melhor qualidade disponível (`-f best`)
- Nome do arquivo: título do vídeo + extensão original

### Systray
- Ícone sempre visível na área de notificação
- Tooltip dinâmico mostrando status e URL atual
- Menu de contexto com opções rápidas

### Permissões
- Acesso completo à área de transferência
- Permissão para mostrar notificações do sistema
- Execução de comandos shell (yt-dlp)

## Solução de Problemas

### "yt-dlp não encontrado"
Verifique se o yt-dlp está no PATH:
```bash
yt-dlp --version
```

### Downloads não iniciam
- Verifique se a pasta de destino existe: `C:\Users\Lucas\Desktop`
- Certifique-se de que você copiou uma URL válida de vídeo

### Aplicativo não aparece no systray
- Reinicie o aplicativo
- Verifique se o systray não está oculto nas configurações do Windows

## Melhorias Futuras Possíveis

- [ ] Configuração de pasta de download customizável
- [ ] Histórico de downloads
- [ ] Opções de qualidade de vídeo
- [ ] Suporte para playlists
- [ ] Interface gráfica para configurações
- [ ] Download automático ao detectar URL (sem necessidade de clicar)

## Licença

Projeto de uso pessoal.