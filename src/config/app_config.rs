use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub models_dir: PathBuf,
    pub data_dir: PathBuf,
    pub output_dir: PathBuf,
    pub training_dir: PathBuf,
    pub db_path: PathBuf,
    pub max_upload_size: usize,
    pub cache_size: usize,
    pub tts_config: TTSConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TTSConfig {
    pub default_sample_rate: u32,
    pub default_format: String,
    pub max_text_length: usize,
    pub max_segment_length: usize,
    pub output_dir: PathBuf,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let models_dir = env::var("MODELS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./models"));

        let data_dir = env::var("DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./data"));

        let output_dir = data_dir.join("audio");
        let training_dir = data_dir.join("training");
        let db_path = data_dir.join("annotations.db");

        let max_upload_size = env::var("MAX_UPLOAD_SIZE")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);

        let cache_size = env::var("CACHE_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let tts_config = TTSConfig {
            default_sample_rate: 22050,
            default_format: "wav".to_string(),
            max_text_length: 10000,
            max_segment_length: 500,
            output_dir: output_dir.clone(),
        };

        Self {
            server_host,
            server_port,
            models_dir,
            data_dir,
            output_dir,
            training_dir,
            db_path,
            max_upload_size,
            cache_size,
            tts_config,
        }
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
