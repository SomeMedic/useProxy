# Запрашиваем права администратора
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Переходим в директорию скрипта
Set-Location $PSScriptRoot

# Компилируем проект в релизном режиме
Write-Host "Compiling the project..." -ForegroundColor Cyan
cargo build --release

# Create the installation directory
$installDir = "$env:ProgramFiles\UseProxy"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Copy the executable file
Copy-Item "target\release\useproxy.exe" -Destination "$installDir\up.exe" -Force

# Add the path to PATH if it's not already there
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "Machine"
    )
}

Write-Host "`nInstallation complete!" -ForegroundColor Green
Write-Host "UseProxy is now available via the 'up' command" -ForegroundColor Green
Write-Host "Restart your terminal to apply the changes" -ForegroundColor Yellow 