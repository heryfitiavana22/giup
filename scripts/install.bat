@echo off
setlocal

set VERSION=v0.1.0
set REPO=heryfitiavana22/gup
set URL=https://github.com/%REPO%/releases/download/%VERSION%/gup-x86_64-pc-windows-gnu.zip
set INSTALLATION_DIR=%LOCALAPPDATA%\Programs\gup\bin

if not exist "%INSTALLATION_DIR%" (
    mkdir "%INSTALLATION_DIR%" 2>nul
    if %ERRORLEVEL% neq 0 (
        echo Run as administrator.
        exit /b 1
    )
)

echo Downloading %URL%...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile 'gup.zip'"

echo Extracting...
powershell -Command "Expand-Archive -Path 'gup.zip' -DestinationPath 'gup-bin'"

echo Installing to %INSTALLATION_DIR%
copy /Y gup-bin\gup.exe "%INSTALLATION_DIR%\gup.exe"

echo %PATH% | find /I "%INSTALLATION_DIR%" >nul
if %ERRORLEVEL% neq 0 (
    setx PATH "%PATH%;%INSTALLATION_DIR%"
)

echo gup installed

del gup.zip
rd /s /q gup-bin

endlocal