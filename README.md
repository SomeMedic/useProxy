```
                                 ██╗   ██╗███████╗███████╗  ██████╗ ██████╗  ██████╗ ██╗  ██╗██╗   ██╗
                                 ██║   ██║██╔════╝██╔════╝  ██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝╚██╗ ██╔╝
                                 ██║   ██║███████╗█████╗    ██████╔╝██████╔╝██║   ██║ ╚███╔╝  ╚████╔╝ 
                                 ██║   ██║╚════██║██╔══╝    ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗   ╚██╔╝  
                                 ╚██████╔╝███████║███████╗  ██║     ██║  ██║╚██████╔╝██╔╝ ██╗   ██║   
                                  ╚═════╝ ╚══════╝╚══════╝  ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   
```

UseProxy - это мощный инструмент для локальной разработки, который позволяет:
- 🔄 Проксировать API-запросы на другие сервера
- 📁 Отдавать статические файлы
- 🔒 Поддерживать HTTPS с самоподписанными сертификатами
- 📝 Логировать и отслеживать запросы

## Быстрая установка

### Windows (PowerShell)

Откройте PowerShell от имени администратора и выполните:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/SomeMedic/useProxy/master/install.ps1'))
```

Или короче:

```powershell
irm https://raw.githubusercontent.com/SomeMedic/useProxy/master/install.ps1 | iex
```

### Ручная установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/SomeMedic/useProxy.git
cd useProxy
```

2. Запустите PowerShell от имени администратора и выполните:
```powershell
.\install.ps1
```

После установки перезапустите терминал, чтобы использовать команду `up`.

## Использование

### Запуск сервера

```bash
# Запуск HTTP сервера
up run

# Запуск HTTPS сервера
up run --https

# Указание порта и хоста
up run --port 8080 --host 0.0.0.0
```

### Управление прокси-правилами

```bash
# Добавление нового правила
up proxy add "/api/chat -> https://api.example.com/chat"

# Добавление правила с изменением Origin
up proxy add "/api/github -> https://api.github.com" --change-origin

# Просмотр всех правил
up proxy list

# Удаление правила
up proxy remove /api/chat

# Очистка всех правил
up proxy clear
```

### Управление сертификатами

```bash
# Генерация сертификатов
up cert gen

# Генерация с указанием путей
up cert gen --cert mycert.pem --key mykey.pem
```

### Управление логами

```bash
# Просмотр логов
up logs show

# Фильтрация логов
up logs filter --method GET --status 404 --path /api

# Очистка логов
up logs clear
```

## Конфигурация

Конфигурация хранится в файле `config.json`. Вы можете указать другой файл конфигурации с помощью параметра `--config`:

```bash
up run --config my-config.json
```

Пример конфигурации:
```json
{
  "host": "127.0.0.1",
  "port": 3000,
  "https": {
    "enabled": true,
    "cert_file": "cert.pem",
    "key_file": "key.pem"
  },
  "static_files": {
    "enabled": true,
    "dir": "./static"
  },
  "proxy": {
    "rules": [
      {
        "path": "/api/github",
        "target": "https://api.github.com",
        "change_origin": true
      }
    ]
  }
}
```

## Требования

- Windows 10/11
- PowerShell 5.1 или выше
- Rust 1.70.0 или выше (для ручной сборки)
