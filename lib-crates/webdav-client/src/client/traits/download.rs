use async_trait::async_trait;

pub enum ThreadMode {
    Auto,
    SingleThread,
    MultipleThread,
}

pub enum TrafficControl {
    Auto,      // 自动控制，根据系统状态动态调整
    Manual,    // 手动控制，由用户或管理员干预
    Throttled, // 限速模式，限制流量以防止过载
}

pub struct DownloadConfig {
    pub thread_mode: ThreadMode,
    pub traffic_control: TrafficControl,
}

impl DownloadConfig {
    pub fn new(
        thread_mode: ThreadMode,
        traffic_control: TrafficControl,
    ) -> Self {
        Self { thread_mode, traffic_control }
    }

    pub fn new_default_config() -> Self {
        Self {
            thread_mode: ThreadMode::Auto,
            traffic_control: TrafficControl::Auto,
        }
    }
}

#[async_trait]
pub trait Download {
    async fn download_file(
        &self,
        files_path: Vec<String>,
        output_path: String,
        download_config: Option<DownloadConfig>,
    ) -> String;
}
