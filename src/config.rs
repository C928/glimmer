use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_host: String,
    pub application_port: u16,
    pub redis_url: String,
    pub api_url: String,
}

impl Settings {
    pub fn new(file_path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::new(file_path, FileFormat::Yaml))
            .build()?
            .try_deserialize()
    }
}
