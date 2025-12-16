@echo off
chcp 65001 >nul
echo ========================================
echo   YT-DLP Monitor - Build Rápido
echo ========================================
echo.

echo [1/4] Encerrando processos anteriores...
taskkill /F /IM ytdl-monitor.exe >nul 2>&1
if %errorlevel% == 0 (
    echo       ✓ Processo encerrado
) else (
    echo       - Nenhum processo rodando
)
echo.

echo [2/4] Limpando cache...
cd src-tauri
cargo clean
echo       ✓ Cache limpo
echo.

echo [3/4] Compilando...
cargo build --release
if %errorlevel% == 0 (
    echo       ✓ Compilação concluída
) else (
    echo       ✗ Erro na compilação
    pause
    exit /b 1
)
echo.

echo [4/4] Build completo!
echo ========================================
echo.
echo 📦 Executável:
echo    target\release\ytdl-monitor.exe
echo.
echo ========================================
echo.

set /p RUN="Deseja executar o aplicativo agora? (S/N): "
if /i "%RUN%"=="S" (
    start "" "target\release\ytdl-monitor.exe"
    echo Aplicativo iniciado!
)

echo.
pause