pub mod api;

use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

const MAX_LOG_ENTRIES: usize = 1000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub path: String,
    pub target_url: String,
    pub status: u16,
    pub duration_ms: u64,
    pub request_headers: Vec<(String, String)>,
    pub response_headers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Logger {
    logs: Arc<Mutex<VecDeque<RequestLog>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            logs: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_LOG_ENTRIES))),
        }
    }

    pub async fn add_log(&self, log: RequestLog) {
        let mut logs = self.logs.lock().await;
        if logs.len() >= MAX_LOG_ENTRIES {
            logs.pop_front();
        }
        logs.push_back(log);
    }

    pub async fn get_logs(&self) -> Vec<RequestLog> {
        let logs = self.logs.lock().await;
        logs.iter().cloned().collect()
    }

    pub async fn clear_logs(&self) {
        let mut logs = self.logs.lock().await;
        logs.clear();
    }

    pub async fn filter_logs(
        &self,
        method: Option<String>,
        status_code: Option<u16>,
        path_contains: Option<String>,
    ) -> Vec<RequestLog> {
        let logs = self.logs.lock().await;
        logs.iter()
            .filter(|log| {
                method.as_ref().map_or(true, |m| log.method == *m)
                    && status_code.map_or(true, |s| log.status == s)
                    && path_contains.as_ref().map_or(true, |p| log.path.contains(p))
            })
            .cloned()
            .collect()
    }
} 