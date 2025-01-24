# Запрашиваем права администратора
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Создаем временную директорию
$tempDir = Join-Path $env:TEMP "UseProxy-Install"
if (Test-Path $tempDir) {
    Remove-Item -Path $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

# Переходим во временную директорию
Set-Location $tempDir

# Скачиваем и распаковываем архив с GitHub
Write-Host "Скачивание UseProxy..." -ForegroundColor Cyan
$repo = "yourusername/useproxy"
$latest = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
$asset = $latest.assets | Where-Object { $_.name -like "*.zip" } | Select-Object -First 1

if ($asset) {
    $downloadUrl = $asset.browser_download_url
    Invoke-WebRequest -Uri $downloadUrl -OutFile "useproxy.zip"
    Expand-Archive "useproxy.zip" -DestinationPath "."
} else {
    Write-Host "Скачивание репозитория..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri "https://github.com/$repo/archive/main.zip" -OutFile "useproxy.zip"
    Expand-Archive "useproxy.zip" -DestinationPath "."
    Move-Item "useproxy-main/*" "." -Force
}

# Создаем директорию для установки
$installDir = "$env:ProgramFiles\UseProxy"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Копируем исполняемый файл
if (Test-Path "up.exe") {
    Copy-Item "up.exe" -Destination "$installDir\up.exe" -Force
} else {
    # Если нет готового исполняемого файла, компилируем
    Write-Host "Компиляция проекта..." -ForegroundColor Cyan
    
    # Проверяем наличие Rust
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Host "Установка Rust..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
        Start-Process -FilePath "rustup-init.exe" -ArgumentList "-y" -Wait
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    }
    
    cargo build --release
    Copy-Item "target\release\useproxy.exe" -Destination "$installDir\up.exe" -Force
}

# Добавляем путь в PATH если его там еще нет
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "Machine"
    )
}

# Очищаем временную директорию
Set-Location $env:TEMP
Remove-Item -Path $tempDir -Recurse -Force

Write-Host "`nУстановка UseProxy завершена!" -ForegroundColor Green
Write-Host "Теперь вы можете использовать команду 'up' из любого места" -ForegroundColor Green
Write-Host "`nПримеры команд:" -ForegroundColor Cyan
Write-Host "up run --https" -ForegroundColor Yellow
Write-Host "up proxy add `"/api/chat -> https://api.example.com`"" -ForegroundColor Yellow
Write-Host "up logs show" -ForegroundColor Yellow
Write-Host "`nПерезапустите терминал, чтобы изменения вступили в силу" -ForegroundColor Yellow 