# Запрашиваем права администратора
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Переходим в директорию скрипта
Set-Location $PSScriptRoot

# Компилируем проект в релизном режиме
Write-Host "Компиляция проекта..." -ForegroundColor Cyan
cargo build --release

# Создаем директорию для установки
$installDir = "$env:ProgramFiles\UseProxy"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Копируем исполняемый файл
Copy-Item "target\release\useproxy.exe" -Destination "$installDir\up.exe" -Force

# Добавляем путь в PATH если его там еще нет
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "Machine"
    )
}

Write-Host "`nУстановка завершена!" -ForegroundColor Green
Write-Host "UseProxy теперь доступен через команду 'up'" -ForegroundColor Green
Write-Host "Перезапустите терминал, чтобы изменения вступили в силу" -ForegroundColor Yellow 