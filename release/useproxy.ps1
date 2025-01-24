# Запрашиваем права администратора
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Переходим в директорию скрипта
Set-Location $PSScriptRoot

# Очищаем экран
Clear-Host

# Устанавливаем заголовок окна
$host.UI.RawUI.WindowTitle = "UseProxy Launcher"

# Глобальная переменная для хранения процесса сервера
$Global:ServerProcess = $null

function Write-VerticalPadding {
    param (
        [int]$lines = 1
    )
    1..$lines | ForEach-Object { Write-Host "" }
}

function Write-CenteredText {
    param (
        [string]$text,
        [string]$foregroundColor = "White",
        [switch]$noNewLine
    )
    
    $screenWidth = $Host.UI.RawUI.WindowSize.Width
    $textLength = $text.Length
    $padding = [math]::Max(0, ($screenWidth - $textLength) / 2)
    
    if ($noNewLine) {
        Write-Host (" " * [math]::Floor($padding)) -NoNewline
        Write-Host $text -ForegroundColor $foregroundColor -NoNewline
    }
    else {
        Write-Host (" " * [math]::Floor($padding)) -NoNewline
        Write-Host $text -ForegroundColor $foregroundColor
    }
}

function Stop-Server {
    if ($Global:ServerProcess -ne $null) {
        Write-VerticalPadding 2
        Write-CenteredText "Stopping server..." "Yellow"
        Stop-Process -Id $Global:ServerProcess.Id -Force
        $Global:ServerProcess = $null
        Write-CenteredText "Server stopped successfully!" "Green"
    }
}

function Start-Server {
    param (
        [string]$mode
    )
    
    # Останавливаем предыдущий сервер, если он запущен
    Stop-Server
    
    Clear-Host
    Write-VerticalPadding 2
    Write-CenteredText "Starting $mode server..." "Cyan"
    
    if ($mode -eq "HTTPS") {
        $Global:ServerProcess = Start-Process cargo -ArgumentList "run", "--", "start", "--port", "3000", "--https" -PassThru -NoNewWindow
    }
    else {
        $Global:ServerProcess = Start-Process cargo -ArgumentList "run", "--", "start", "--port", "3000" -PassThru -NoNewWindow
    }
    
    Write-CenteredText "Server is running in background (PID: $($Global:ServerProcess.Id))" "Green"
    Write-VerticalPadding 2
    Write-CenteredText "Press Enter to return to menu..." "Gray"
    Read-Host
}

function Show-Menu {
    Clear-Host
    Write-VerticalPadding 2
    Write-CenteredText "                 _____                     " "Cyan"
    Write-CenteredText "                |  __ \                    " "Cyan"
    Write-CenteredText "  _   _ ___  ___| |__) | __ _____  ___   _ " "Cyan"
    Write-CenteredText " | | | / __|/ _ \  ___/ '__/ _ \ \/ / | | |" "Cyan"
    Write-CenteredText " | |_| \__ \  __/ |   | | | (_) >  <| |_| |" "Cyan"
    Write-CenteredText "  \__,_|___/\___|_|   |_|  \___/_/\_\\__, |" "Cyan"
    Write-CenteredText "                                      __/ |" "Cyan"
    Write-CenteredText "                                     |___/ " "Cyan"
    Write-VerticalPadding 1
    Write-CenteredText "+===================================================+" "Blue"
    Write-VerticalPadding 1
    Write-CenteredText "[1] Start HTTPS Server"
    Write-VerticalPadding 1
    Write-CenteredText "[2] Start HTTP Server"
    Write-VerticalPadding 1
    Write-CenteredText "[3] Proxy Rules Management"
    Write-VerticalPadding 1
    Write-CenteredText "[4] Generate Certificates"
    Write-VerticalPadding 1
    Write-CenteredText "[5] View Logs"
    Write-VerticalPadding 1
    Write-CenteredText "[6] Stop Server"
    Write-VerticalPadding 1
    Write-CenteredText "[Q] Exit"
    Write-VerticalPadding 1
    Write-CenteredText "+===================================================+" "Blue"
    Write-VerticalPadding 2
    
    if ($Global:ServerProcess -ne $null -and -not $Global:ServerProcess.HasExited) {
        Write-CenteredText "[STATUS] Server is running (PID: $($Global:ServerProcess.Id))" "Green"
    }
    else {
        Write-CenteredText "[STATUS] Server is stopped" "Red"
    }
}

function Show-ProxyMenu {
    Clear-Host
    Write-VerticalPadding 3
    Write-CenteredText "============= Proxy Rules Management =============" "Magenta"
    Write-VerticalPadding 1
    Write-CenteredText "[1] Add new rule"
    Write-VerticalPadding 1
    Write-CenteredText "[2] Remove rule"
    Write-VerticalPadding 1
    Write-CenteredText "[3] List all rules"
    Write-VerticalPadding 1
    Write-CenteredText "[4] Clear all rules"
    Write-VerticalPadding 1
    Write-CenteredText "[B] Back to main menu"
    Write-VerticalPadding 1
    Write-CenteredText "================================================" "Magenta"
}

$choice = ""
while ($choice -ne "Q") {
    Show-Menu
    Write-VerticalPadding 2
    Write-CenteredText "Select action: " "Yellow" -noNewLine
    $choice = Read-Host
    
    if ($choice -eq "1") {
        Start-Server "HTTPS"
    }
    elseif ($choice -eq "2") {
        Start-Server "HTTP"
    }
    elseif ($choice -eq "3") {
        do {
            Show-ProxyMenu
            Write-VerticalPadding 2
            Write-CenteredText "Select action: " "Yellow" -noNewLine
            $proxyChoice = Read-Host
            
            if ($proxyChoice -eq "1") {
                Write-VerticalPadding 2
                Write-CenteredText "=== Adding New Proxy Rule ===" "Cyan"
                Write-VerticalPadding 1
                Write-CenteredText "Enter path (e.g. /api/github): " "Yellow" -noNewLine
                $path = Read-Host
                Write-CenteredText "Enter target URL (e.g. https://api.github.com): " "Yellow" -noNewLine
                $target = Read-Host
                Write-CenteredText "Change Origin header? (y/n): " "Yellow" -noNewLine
                $changeOrigin = Read-Host
                
                if ($changeOrigin -eq "y") {
                    cargo run -- proxy add $path $target --change-origin
                }
                else {
                    cargo run -- proxy add $path $target
                }
                Write-VerticalPadding 2
                Write-CenteredText "Press Enter to continue..." "Gray"
                Read-Host
            }
            elseif ($proxyChoice -eq "2") {
                Write-VerticalPadding 2
                Write-CenteredText "=== Removing Proxy Rule ===" "Cyan"
                Write-VerticalPadding 1
                Write-CenteredText "Enter path to remove: " "Yellow" -noNewLine
                $path = Read-Host
                cargo run -- proxy remove $path
                Write-VerticalPadding 2
                Write-CenteredText "Press Enter to continue..." "Gray"
                Read-Host
            }
            elseif ($proxyChoice -eq "3") {
                Write-VerticalPadding 2
                Write-CenteredText "=== Current Proxy Rules ===" "Cyan"
                Write-VerticalPadding 1
                cargo run -- proxy list
                Write-VerticalPadding 2
                Write-CenteredText "Press Enter to continue..." "Gray"
                Read-Host
            }
            elseif ($proxyChoice -eq "4") {
                Write-VerticalPadding 2
                Write-CenteredText "=== Clearing All Rules ===" "Cyan"
                Write-VerticalPadding 1
                cargo run -- proxy clear
                Write-VerticalPadding 2
                Write-CenteredText "Press Enter to continue..." "Gray"
                Read-Host
            }
        } while ($proxyChoice -ne "B")
    }
    elseif ($choice -eq "4") {
        Clear-Host
        Write-VerticalPadding 3
        Write-CenteredText "=== Generating Certificates ===" "Cyan"
        Write-VerticalPadding 1
        cargo run -- gen-cert
        Write-VerticalPadding 2
        Write-CenteredText "Press Enter to continue..." "Gray"
        Read-Host
    }
    elseif ($choice -eq "5") {
        Clear-Host
        Write-VerticalPadding 3
        Write-CenteredText "=== View Logs ===" "Cyan"
        Write-VerticalPadding 1
        Write-CenteredText "To view logs, open in browser: " -noNewLine
        Write-Host "https://localhost:3000/logs" -ForegroundColor Yellow
        Write-VerticalPadding 2
        Write-CenteredText "Press Enter to continue..." "Gray"
        Read-Host
    }
    elseif ($choice -eq "6") {
        Stop-Server
        Write-VerticalPadding 2
        Write-CenteredText "Press Enter to continue..." "Gray"
        Read-Host
    }
}

# Останавливаем сервер при выходе
Stop-Server 