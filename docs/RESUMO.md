# 🎬 YT-DLP Monitor - Resumo do Projeto

## ✅ O Que Foi Criado

Um aplicativo completo em Tauri para Windows 11 que:

### Funcionalidades Principais
1. ✅ **Monitora área de transferência** - Detecta automaticamente URLs de vídeo copiadas
2. ✅ **Roda no systray** - Fica na bandeja do sistema, não abre janela
3. ✅ **Baixa com um clique** - Clique esquerdo no ícone para iniciar download
4. ✅ **Salva na área de trabalho** - Downloads vão para `C:\Users\Lucas\Desktop`
5. ✅ **Usa yt-dlp** - Aproveita sua instalação existente do yt-dlp
6. ✅ **Notificações visuais** - Feedback claro do que está acontecendo

### Plataformas Suportadas
- YouTube (youtube.com, youtu.be)
- Vimeo
- Dailymotion
- Twitch
- TikTok
- Instagram
- Facebook
- Twitter/X

## 📁 Estrutura do Projeto

```
ytdl-monitor/
│
├── 📄 README.md              - Documentação principal
├── 📄 INSTALL.md             - Guia de instalação passo a passo
├── 📄 TROUBLESHOOTING.md     - Solução de problemas detalhada
├── 📄 ADVANCED.md            - Ideias para melhorias futuras
├── 📄 .gitignore             - Arquivo Git
├── 🔧 build.bat              - Script automático de compilação
├── 🔧 dev.bat                - Script para modo desenvolvimento
│
├── src/                      - Frontend (HTML simples)
│   └── index.html
│
└── src-tauri/                - Backend Rust/Tauri
    ├── 📄 Cargo.toml         - Dependências Rust
    ├── 📄 tauri.conf.json    - Configuração do Tauri
    ├── 📄 build.rs           - Script de build
    │
    ├── src/
    │   └── 🦀 main.rs        - Código principal (370 linhas)
    │
    └── icons/                - Pasta para ícones
        └── 📄 README.md      - Instruções sobre ícones
```

## 🚀 Como Usar (Início Rápido)

### Passo 1: Instalar Rust
```bash
# Baixe de: https://www.rust-lang.org/tools/install
# Execute rustup-init.exe
```

### Passo 2: Gerar Ícones
```bash
# Com uma imagem PNG quadrada (512x512 ou maior)
cargo tauri icon sua-imagem.png
```

### Passo 3: Compilar
```bash
# Opção fácil:
build.bat

# Ou manualmente:
cd src-tauri
cargo tauri build
```

### Passo 4: Executar
```
Executável em: src-tauri\target\release\ytdl-monitor.exe
```

## 💡 Como Funciona

1. **Inicialização**
   - App inicia e fica no systray
   - Thread em background monitora área de transferência a cada 500ms

2. **Detecção de URL**
   - Quando você copia uma URL de vídeo
   - App detecta automaticamente
   - Mostra notificação: "URL de Vídeo Detectada"
   - Atualiza tooltip do systray com a URL

3. **Download**
   - Clique esquerdo no ícone do systray
   - yt-dlp é executado em background
   - Notificação mostra progresso
   - Vídeo salvo em `C:\Users\Lucas\Desktop`

## 🔧 Tecnologias Utilizadas

- **Tauri 1.5** - Framework para desktop apps
- **Rust** - Backend performático
- **clipboard-win** - Acesso à área de transferência do Windows
- **yt-dlp** - Download dos vídeos (executado via command)
- **Tokio** - Runtime async para Rust

## 📋 Arquivos Importantes

### main.rs (Código Principal)
- ✅ Monitoramento da área de transferência
- ✅ Detecção de URLs de vídeo
- ✅ Integração com yt-dlp
- ✅ Sistema de notificações
- ✅ Menu do systray
- ✅ Gestão de estado (última URL, status download)

### Cargo.toml (Dependências)
```toml
tauri = "1.5"              # Framework
serde = "1.0"              # Serialização
tokio = "1.0"              # Async runtime
clipboard-win = "5.0"      # Área de transferência
```

### tauri.conf.json (Configurações)
- Permissões de clipboard
- Configuração do systray
- Janela invisível (só systray)
- Bundle settings

## 🎯 Próximos Passos Recomendados

1. **Criar/Obter Ícones**
   - Use Flaticon, Icons8, ou crie um
   - Execute `cargo tauri icon sua-imagem.png`

2. **Primeira Compilação**
   - Execute `build.bat`
   - Aguarde 10-20 minutos (primeira vez)

3. **Testar**
   - Execute o .exe gerado
   - Copie uma URL do YouTube
   - Clique no ícone do systray

4. **Personalizar** (Opcional)
   - Edite `main.rs` para mudar pasta de download
   - Adicione novos sites na função `is_video_url()`
   - Customize as notificações

## 📚 Documentação Incluída

- **README.md** - Visão geral e funcionalidades
- **INSTALL.md** - Guia passo a passo de instalação
- **TROUBLESHOOTING.md** - Solução de 20+ problemas comuns
- **ADVANCED.md** - Ideias para versões futuras

## ⚠️ Notas Importantes

### Antes de Compilar
- ✅ Rust instalado
- ✅ Ícones gerados (pasta icons/)
- ✅ yt-dlp no PATH (você já tem!)

### Durante Uso
- ✅ App roda em background
- ✅ Baixa um vídeo por vez
- ✅ Notificações mostram status
- ✅ Clique esquerdo = download
- ✅ Menu botão direito = opções

### Limitações Atuais
- ⚠️ Um download por vez
- ⚠️ Sem interface de configuração
- ⚠️ Sem histórico de downloads
- ⚠️ Sem barra de progresso visual

(Veja ADVANCED.md para planos futuros)

## 🛠️ Customizações Fáceis

### Mudar Pasta de Download
```rust
// Em main.rs, linha ~50
let download_path = "C:\\Users\\Lucas\\Desktop";
// Altere para:
let download_path = "C:\\Meus Videos";
```

### Adicionar Novo Site
```rust
// Em main.rs, função is_video_url()
let video_domains = [
    "youtube.com",
    "seunovosite.com",  // Adicione aqui
    // ...
];
```

### Mudar Qualidade do Download
```rust
// Em main.rs, função download_video()
.arg("-f")
.arg("best")  // Altere para: "bestvideo+bestaudio"
```

## 🎓 Para Aprender Mais

- **Tauri:** https://tauri.app/
- **Rust:** https://www.rust-lang.org/learn
- **yt-dlp:** https://github.com/yt-dlp/yt-dlp

## 📞 Suporte

Se algo não funcionar:
1. Consulte TROUBLESHOOTING.md
2. Verifique INSTALL.md
3. Execute em modo dev: `dev.bat`
4. Veja logs no terminal

---

**Versão:** 0.1.0  
**Plataforma:** Windows 11  
**Status:** ✅ Pronto para compilar e usar!

**Tempo estimado até estar usando:**
- Com Rust já instalado: 15-25 minutos
- Sem Rust: 30-45 minutos (incluindo instalação)