@echo off
REM Script de desenvolvimento para Windows

echo ====================================
echo   YT-DLP Monitor - Dev Mode
echo ====================================
echo.

echo Verificando yt-dlp...
yt-dlp --version
if errorlevel 1 (
    echo [AVISO] yt-dlp nao encontrado no PATH!
    echo O aplicativo pode nao funcionar corretamente.
    echo.
)

echo Verificando Rust...
cargo --version
if errorlevel 1 (
    echo [ERRO] Rust/Cargo nao encontrado!
    echo Instale de: https://www.rust-lang.org/tools/install
    pause
    exit /b 1
)

echo.
echo Iniciando modo desenvolvimento...
echo Pressione Ctrl+C para encerrar.
echo.

cd src
cargo tauri dev