@echo off
rem Builds PIXORA.exe on Windows from this source folder.
rem Needs Rust (https://rustup.rs, the default MSVC option; it offers the Visual Studio Build Tools).
setlocal
cd /d "%~dp0"
where cargo >nul 2>nul
if errorlevel 1 (
  echo Rust isn't installed. Get it from https://rustup.rs ^(default options^), then run this again.
  pause
  exit /b 1
)
set "CRAFT_FONTS_DIR=%~dp0packaging\pixora"
for /f %%d in ('powershell -NoProfile -Command "Get-Date -Format yyyy-MM-dd"') do set "PHOTOCRAFT_BUILD_DATE=%%d"
echo Building PIXORA (the first build takes a while) ...
cargo build --release -p photocraft -p photocraft-cli --features photocraft/heif,photocraft-cli/heif
if errorlevel 1 (
  echo Build failed.
  pause
  exit /b 1
)
set "OUT=%~dp0dist\PIXORA"
if not exist "%OUT%" mkdir "%OUT%"
copy /y "target\release\photocraft.exe" "%OUT%\PIXORA.exe" >nul
copy /y "target\release\photocraft-cli.exe" "%OUT%\pixora-cli.exe" >nul
copy /y LICENSE-MIT "%OUT%" >nul
copy /y LICENSE-APACHE "%OUT%" >nul
copy /y NOTICE "%OUT%" >nul
copy /y ATTRIBUTION.md "%OUT%" >nul
copy /y "packaging\pixora\fonts\OFL-NotoSinhala.txt" "%OUT%" >nul
copy /y "packaging\pixora\README-PIXORA.txt" "%OUT%\README.txt" >nul
copy /y "packaging\pixora\Create Desktop Shortcut.bat" "%OUT%" >nul
echo.
echo Done: %OUT%\PIXORA.exe
pause
