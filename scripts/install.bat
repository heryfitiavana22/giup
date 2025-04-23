@echo off
setlocal

set VERSION=v0.1.1
set REPO=heryfitiavana22/giup
set URL=https://github.com/%REPO%/releases/download/%VERSION%/giup-x86_64-pc-windows-gnu.zip
set INSTALLATION_DIR=%LOCALAPPDATA%\Programs\giup\bin

if not exist "%INSTALLATION_DIR%" (
    mkdir "%INSTALLATION_DIR%" 2>nul
    if %ERRORLEVEL% neq 0 (
        echo Run as administrator.
        exit /b 1
    )
)

echo Downloading %URL%...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile 'giup.zip'"

echo Extracting...
powershell -Command "Expand-Archive -Path 'giup.zip' -DestinationPath 'giup-bin'"

echo Installing to %INSTALLATION_DIR%
copy /Y giup-bin\giup.exe "%INSTALLATION_DIR%\giup.exe"

echo %PATH% | find /I "%INSTALLATION_DIR%" >nul
if %ERRORLEVEL% neq 0 (
    setx PATH "%PATH%;%INSTALLATION_DIR%"
)

echo giup installed

del giup.zip
rd /s /q giup-bin

endlocal