# Guia de Instalação Rápida - YT-DLP Monitor

## Pré-requisitos

### 1. yt-dlp (JÁ INSTALADO ✅)
- Você já tem o yt-dlp instalado e configurado no PATH

### 2. Instalar Rust

1. Baixe o instalador do Rust:
   - Acesse: https://www.rust-lang.org/tools/install
   - Baixe `rustup-init.exe`

2. Execute o instalador:
   - Aceite as opções padrão
   - Aguarde a instalação completa (pode demorar alguns minutos)

3. Verifique a instalação abrindo um novo terminal:
   ```bash
   rustc --version
   cargo --version
   ```

### 3. Instalar Tauri CLI (Opcional, mas recomendado)

```bash
cargo install tauri-cli
```

## Passos para Compilar

### Método 1: Usando o Script (Recomendado)

1. Abra o terminal na pasta do projeto
2. Execute:
   ```bash
   build.bat
   ```
3. Aguarde a compilação
4. O executável estará em: `src\target\release\ytdl-monitor.exe`

### Método 2: Manual

1. Abra o terminal na pasta do projeto
2. Navegue até src:
   ```bash
   cd src
   ```
3. Compile:
   ```bash
   cargo tauri build
   ```

## Após Compilar

1. Navegue até: `src\target\release\`
2. Execute `ytdl-monitor.exe`
3. O aplicativo aparecerá no systray (área de notificação)

## Gerando Ícones

Antes de compilar, você precisa dos ícones. Opções:

### Opção A: Usar o gerador do Tauri (Recomendado)

1. Tenha uma imagem PNG quadrada (512x512 ou maior)
2. Execute:
   ```bash
   cargo tauri icon caminho/para/seu/icone.png
   ```

### Opção B: Download de ícones prontos

Baixe de sites como:
- https://www.flaticon.com/ (busque por "download" ou "video")
- https://icons8.com/

Renomeie e coloque em `src-tauri/icons/`:
- `icon.ico` (obrigatório para Windows)
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`

## Solução de Problemas Comuns

### "Rust não encontrado"
- Certifique-se de ter fechado e reaberto o terminal após instalar o Rust
- Reinicie o computador se necessário

### "Erro de compilação"
- Verifique se tem todos os ícones necessários
- Tente executar: `cargo clean` e compile novamente

### "Aplicativo não inicia"
- Verifique o Windows Defender (pode estar bloqueando)
- Execute como administrador

## Próximos Passos

1. Após compilar, copie o `ytdl-monitor.exe` para onde preferir
2. Crie um atalho na pasta de inicialização para executar automaticamente:
   - Pressione `Win + R`
   - Digite: `shell:startup`
   - Copie o atalho do executável para esta pasta

## Uso

1. Execute o aplicativo
2. Copie qualquer URL de vídeo (YouTube, TikTok, etc.)
3. Clique no ícone do systray para baixar
4. Vídeo será salvo em: `C:\Users\Lucas\Desktop`

---

**Tempo estimado de primeira compilação:** 10-20 minutos (depende do seu PC)
**Compilações subsequentes:** 1-3 minutos