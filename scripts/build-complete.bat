@echo off
chcp 65001 >nul
echo ========================================
echo   YT-DLP Monitor - Build Completo
echo ========================================
echo.

REM Cores (simuladas com mensagens)
echo [1/5] Encerrando processos anteriores...
taskkill /F /IM ytdl-monitor.exe >nul 2>&1
if %errorlevel% == 0 (
    echo       ✓ Processo encerrado
) else (
    echo       - Nenhum processo rodando
)
echo.

echo [2/5] Limpando cache de compilação...
cd ..\src-tauri
cargo clean
if %errorlevel% == 0 (
    echo       ✓ Cache limpo
) else (
    echo       ✗ Erro ao limpar cache
    pause
    exit /b 1
)
echo.

echo [3/5] Compilando em modo Release...
echo       (Isso pode demorar alguns minutos...)
cargo build --release
if %errorlevel% == 0 (
    echo       ✓ Compilação concluída
) else (
    echo       ✗ Erro na compilação
    pause
    exit /b 1
)
echo.

echo [4/5] Gerando instalador NSIS...
echo       (Primeira vez pode demorar 10-15 minutos...)
cargo tauri build
if %errorlevel% == 0 (
    echo       ✓ Instalador gerado
) else (
    echo       ✗ Erro ao gerar instalador
    pause
    exit /b 1
)
echo.

echo [5/5] Build completo!
echo ========================================
echo.
echo 📦 Arquivos gerados:
echo.
echo    Executável:
echo    target\release\ytdl-monitor.exe
echo.
echo    Instalador:
echo    target\release\bundle\nsis\YT-DLP Monitor_0.1.0_x64-setup.exe
echo.
echo ========================================
echo.

REM Perguntar se quer executar
set /p RUN="Deseja executar o aplicativo agora? (S/N): "
if /i "%RUN%"=="S" (
    echo.
    echo Executando...
    start "" "target\release\ytdl-monitor.exe"
    echo.
    echo Aplicativo iniciado no systray!
) else (
    echo.
    echo Você pode executar manualmente:
    echo target\release\ytdl-monitor.exe
)

echo.
pause