use clap::{Parser, Subcommand};
use std::path::PathBuf;

const ABOUT: &str = "\
██╗   ██╗███████╗███████╗██████╗ ██████╗  ██████╗ ██╗  ██╗██╗   ██╗
██║   ██║██╔════╝██╔════╝██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝╚██╗ ██╔╝
██║   ██║███████╗█████╗  ██████╔╝██████╔╝██║   ██║ ╚███╔╝  ╚████╔╝ 
██║   ██║╚════██║██╔══╝  ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗   ╚██╔╝  
╚██████╔╝███████║███████╗██║     ██║  ██║╚██████╔╝██╔╝ ██╗   ██║   
 ╚═════╝ ╚══════╝╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   

UseProxy - Универсальный прокси для локальной разработки";

const LONG_ABOUT: &str = "\
██╗   ██╗███████╗███████╗██████╗ ██████╗  ██████╗ ██╗  ██╗██╗   ██╗
██║   ██║██╔════╝██╔════╝██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝╚██╗ ██╔╝
██║   ██║███████╗█████╗  ██████╔╝██████╔╝██║   ██║ ╚███╔╝  ╚████╔╝ 
██║   ██║╚════██║██╔══╝  ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗   ╚██╔╝  
╚██████╔╝███████║███████╗██║     ██║  ██║╚██████╔╝██╔╝ ██╗   ██║   
 ╚═════╝ ╚══════╝╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   

UseProxy - это инструмент для локальной разработки, который позволяет:
- Проксировать API-запросы на другие сервера
- Отдавать статические файлы
- Поддерживать HTTPS с самоподписанными сертификатами
- Логировать и отслеживать запросы";

#[derive(Parser)]
#[command(
    name = "up",
    about = ABOUT,
    version,
    author,
    long_about = LONG_ABOUT
)]
pub struct Cli {
    /// Путь к файлу конфигурации (по умолчанию: config.json)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Запустить сервер
    Run {
        /// Порт для прослушивания (по умолчанию: 3000)
        #[arg(short, long)]
        port: Option<u16>,
        
        /// Хост для прослушивания (по умолчанию: 127.0.0.1)
        #[arg(short = 'H', long)]
        host: Option<String>,
        
        /// Включить HTTPS
        #[arg(long)]
        https: bool,
    },
    
    /// Управление прокси-правилами
    Proxy {
        #[command(subcommand)]
        command: ProxyCommands,
    },

    /// Управление сертификатами
    Cert {
        #[command(subcommand)]
        command: CertCommands,
    },

    /// Управление логами
    Logs {
        #[command(subcommand)]
        command: LogCommands,
    },
}

#[derive(Subcommand)]
pub enum ProxyCommands {
    /// Добавить новое правило
    /// Пример: up proxy add "/api/github -> https://api.github.com"
    /// Важно: правило должно быть в кавычках!
    Add {
        /// Правило в формате "путь -> целевой_url" (в кавычках)
        rule: String,
        
        /// Изменять Origin заголовок
        #[arg(long)]
        change_origin: bool,
    },
    
    /// Удалить правило
    Remove {
        /// Путь правила для удаления
        path: String,
    },
    
    /// Показать все правила
    List,
    
    /// Очистить все правила
    Clear,
}

#[derive(Subcommand)]
pub enum CertCommands {
    /// Сгенерировать новые сертификаты
    Gen {
        /// Путь для сохранения сертификата
        #[arg(long, value_name = "FILE")]
        cert: Option<PathBuf>,
        
        /// Путь для сохранения ключа
        #[arg(long, value_name = "FILE")]
        key: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum LogCommands {
    /// Показать логи
    Show,
    
    /// Очистить логи
    Clear,
    
    /// Фильтровать логи
    Filter {
        /// HTTP метод
        #[arg(long)]
        method: Option<String>,
        
        /// Статус код
        #[arg(long)]
        status: Option<u16>,
        
        /// Путь запроса
        #[arg(long)]
        path: Option<String>,
    },
} 