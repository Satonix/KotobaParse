@echo off
setlocal EnableExtensions

rem Coloque este arquivo na raiz do KotobaParse e execute-o.
cd /d "%~dp0"

where tar.exe >nul 2>&1
if errorlevel 1 (
    echo [ERRO] O tar.exe nao foi encontrado.
    echo Ele acompanha as versoes atuais do Windows 10 e 11.
    pause
    exit /b 1
)

for %%I in (.) do set "PROJECT_NAME=%%~nxI"
for /f %%I in ('powershell.exe -NoProfile -Command "Get-Date -Format yyyyMMdd-HHmmss"') do set "STAMP=%%I"

set "PACKAGE_DIR=%CD%\packages"
set "OUTPUT=%PACKAGE_DIR%\%PROJECT_NAME%-%STAMP%.zip"
set "TEMP_OUTPUT=%TEMP%\%PROJECT_NAME%-%STAMP%-%RANDOM%.zip"

if not exist "%PACKAGE_DIR%" mkdir "%PACKAGE_DIR%"

echo.
echo Compactando %PROJECT_NAME%...
echo Destino: "%OUTPUT%"
echo.

tar.exe -a -c -f "%TEMP_OUTPUT%" ^
    --exclude="./packages" ^
    --exclude="*/packages" ^
    --exclude="./target" ^
    --exclude="*/target" ^
    --exclude="./dist" ^
    --exclude="*/dist" ^
    --exclude="./build" ^
    --exclude="*/build" ^
    --exclude="./coverage" ^
    --exclude="*/coverage" ^
    --exclude="./.git" ^
    --exclude="*/.git" ^
    --exclude="./.env" ^
    --exclude="*/.env" ^
    --exclude="./.env.local" ^
    --exclude="*/.env.local" ^
    --exclude="*.zip" ^
    --exclude="*.7z" ^
    .

if errorlevel 1 (
    echo.
    echo [ERRO] Nao foi possivel gerar o ZIP.
    if exist "%TEMP_OUTPUT%" del /q "%TEMP_OUTPUT%" >nul 2>&1
    pause
    exit /b 1
)

move /y "%TEMP_OUTPUT%" "%OUTPUT%" >nul
if errorlevel 1 (
    echo.
    echo [ERRO] O ZIP foi criado, mas nao pode ser movido para a pasta packages.
    if exist "%TEMP_OUTPUT%" del /q "%TEMP_OUTPUT%" >nul 2>&1
    pause
    exit /b 1
)

for %%I in ("%OUTPUT%") do set "SIZE=%%~zI"

echo.
echo [OK] ZIP criado:
echo "%OUTPUT%"
echo Tamanho: %SIZE% bytes
echo.
pause
exit /b 0
