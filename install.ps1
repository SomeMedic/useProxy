# Запрашиваем права администратора
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Функция для очистки и выхода
function Cleanup-AndExit {
    param (
        [string]$ErrorMessage
    )
    
    if (Test-Path $tempDir) {
        Set-Location $env:TEMP
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    if ($ErrorMessage) {
        Write-Host "`n$ErrorMessage" -ForegroundColor Red
        exit 1
    }
}

# Создаем временную директорию
$tempDir = Join-Path $env:TEMP "UseProxy-Install"
if (Test-Path $tempDir) {
    Remove-Item -Path $tempDir -Recurse -Force
}

try {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
    Set-Location $tempDir
} catch {
    Cleanup-AndExit "Не удалось создать временную директорию: $_"
}

# Скачиваем и распаковываем архив с GitHub
Write-Host "Скачивание UseProxy..." -ForegroundColor Cyan
$repo = "SomeMedic/useProxy"

try {
    Write-Host "Скачивание репозитория..." -ForegroundColor Cyan
    $downloadUrl = "https://github.com/$repo/archive/master.zip"
    Invoke-WebRequest -Uri $downloadUrl -OutFile "useproxy.zip"
    Expand-Archive "useproxy.zip" -DestinationPath "." -Force
    
    # Находим директорию с исходным кодом
    $sourceDir = Get-ChildItem -Directory | Where-Object { $_.Name -like "*useProxy-*" } | Select-Object -First 1
    if (-not $sourceDir) {
        throw "Не удалось найти директорию с исходным кодом"
    }
    
    Set-Location $sourceDir.FullName
} catch {
    Cleanup-AndExit "Ошибка при скачивании/распаковке: $_"
}

# Проверяем наличие Rust
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Установка Rust..." -ForegroundColor Yellow
    try {
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
        Start-Process -FilePath "rustup-init.exe" -ArgumentList "-y" -Wait
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    } catch {
        Cleanup-AndExit "Ошибка при установке Rust: $_"
    }
}

# Компилируем проект
Write-Host "Компиляция проекта..." -ForegroundColor Cyan
try {
    cargo build --release
    if (-not (Test-Path "target/release/useproxy.exe")) {
        throw "Не удалось найти скомпилированный файл"
    }
} catch {
    Cleanup-AndExit "Ошибка при компиляции: $_"
}

# Создаем директорию для установки
$installDir = "$env:ProgramFiles\UseProxy"
try {
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir | Out-Null
    }
    Copy-Item "target/release/useproxy.exe" -Destination "$installDir\up.exe" -Force
} catch {
    Cleanup-AndExit "Ошибка при копировании файлов: $_"
}

# Добавляем путь в PATH
try {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$currentPath;$installDir",
            "Machine"
        )
    }
} catch {
    Cleanup-AndExit "Ошибка при обновлении PATH: $_"
}

# Очищаем временные файлы
Cleanup-AndExit

Write-Host "`nУстановка UseProxy завершена!" -ForegroundColor Green
Write-Host "Теперь вы можете использовать команду 'up' из любого места" -ForegroundColor Green
Write-Host "`nПримеры команд:" -ForegroundColor Cyan
Write-Host "up run --https" -ForegroundColor Yellow
Write-Host "up proxy add `"/api/chat -> https://api.example.com`"" -ForegroundColor Yellow
Write-Host "up logs show" -ForegroundColor Yellow
Write-Host "`nПерезапустите терминал, чтобы изменения вступили в силу" -ForegroundColor Yellow 