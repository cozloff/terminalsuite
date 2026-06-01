#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Probe,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: std::env::var("XR_APP_NAME")
                .unwrap_or_else(|_| "TerminalSuite XR Prototype".to_string()),
            mode: Mode::Probe,
        }
    }
}
