use crate::error::{AppError, Result};
use crate::python_bridge::TTSBridge;
use crate::config::TTSConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;
use std::num::NonZeroUsize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizeRequest {
    pub text: String,
    pub model_path: String,
    pub emod_path: Option<String>,
    pub sample_rate: Option<u32>,
    pub output_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizeResponse {
    pub audio_id: String,
    pub audio_path: String,
    pub duration: f64,
    pub sample_rate: u32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionParams {
    pub valence: Option<f32>,
    pub arousal: Option<f32>,
    pub dominance: Option<f32>,
    pub speaking_rate: Option<f32>,
    pub pitch_offset: Option<f32>,
}

pub struct TTSService {
    bridge: Arc<RwLock<Option<TTSBridge>>>,
    loaded_models: Arc<RwLock<LruCache<String, ()>>>,
    output_dir: PathBuf,
    config: TTSConfig,
}

impl TTSService {
    pub fn new(config: TTSConfig) -> Self {
        let cache_size = NonZeroUsize::new(10).unwrap();
        
        Self {
            bridge: Arc::new(RwLock::new(None)),
            loaded_models: Arc::new(RwLock::new(LruCache::new(cache_size))),
            output_dir: config.output_dir.clone(),
            config,
        }
    }

    pub async fn initialize_bridge(&self) -> Result<()> {
        let bridge = TTSBridge::new()
            .map_err(|e| AppError::PythonError(format!("Failed to initialize TTS bridge: {}", e)))?;
        
        let mut bridge_lock = self.bridge.write().await;
        *bridge_lock = Some(bridge);
        
        Ok(())
    }

    pub async fn synthesize(&self, request: SynthesizeRequest) -> Result<SynthesizeResponse> {
        let text = request.text;
        if text.is_empty() {
            return Err(AppError::ValidationError("Text cannot be empty".to_string()));
        }
        
        if text.len() > self.config.max_text_length {
            return Err(AppError::ValidationError(format!(
                "Text length exceeds maximum allowed length of {} characters",
                self.config.max_text_length
            )));
        }

        let audio_id = Uuid::new_v4().to_string();
        let format = request.output_format.unwrap_or_else(|| self.config.default_format.clone());
        let sample_rate = request.sample_rate.unwrap_or(self.config.default_sample_rate);
        
        let output_path = self.output_dir
            .join(format!("{}.{}", audio_id, format));

        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("TTS bridge not initialized".to_string()))?;

        bridge.synthesize(
            &text,
            &request.model_path,
            request.emod_path.as_deref(),
            output_path.to_str().unwrap(),
        ).map_err(|e| AppError::SynthesisError(format!("Synthesis failed: {}", e)))?;

        let duration = TTSBridge::get_audio_duration(&output_path)
            .map_err(|e| AppError::IoError(format!("Failed to get audio duration: {}", e)))?;

        Ok(SynthesizeResponse {
            audio_id,
            audio_path: output_path.to_string_lossy().to_string(),
            duration,
            sample_rate,
            format,
        })
    }

    pub async fn synthesize_with_emotion(
        &self,
        text: &str,
        model_path: &str,
        emotion_params: &EmotionParams,
    ) -> Result<SynthesizeResponse> {
        let params_json = serde_json::to_string(emotion_params)
            .map_err(|e| AppError::ValidationError(format!("Invalid emotion params: {}", e)))?;

        let audio_id = Uuid::new_v4().to_string();
        let format = self.config.default_format.clone();
        let output_path = self.output_dir.join(format!("{}.{}", audio_id, format));

        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("TTS bridge not initialized".to_string()))?;

        bridge.synthesize_with_params(
            text,
            model_path,
            output_path.to_str().unwrap(),
            Some(&params_json),
        ).map_err(|e| AppError::SynthesisError(format!("Synthesis failed: {}", e)))?;

        let duration = TTSBridge::get_audio_duration(&output_path)
            .map_err(|e| AppError::IoError(format!("Failed to get audio duration: {}", e)))?;

        Ok(SynthesizeResponse {
            audio_id,
            audio_path: output_path.to_string_lossy().to_string(),
            duration,
            sample_rate: self.config.default_sample_rate,
            format,
        })
    }

    pub async fn preload_model(&self, model_path: &str) -> Result<()> {
        let model_id = model_path.to_string();
        
        {
            let cache = self.loaded_models.read().await;
            if cache.contains(&model_id) {
                return Ok(());
            }
        }

        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("TTS bridge not initialized".to_string()))?;

        bridge.load_model(model_path)
            .map_err(|e| AppError::ModelError(format!("Failed to load model: {}", e)))?;

        let mut cache = self.loaded_models.write().await;
        cache.put(model_id, ());

        Ok(())
    }

    pub async fn get_model_info(&self, model_path: &str) -> Result<serde_json::Value> {
        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("TTS bridge not initialized".to_string()))?;

        bridge.get_model_info(model_path)
            .map_err(|e| AppError::ModelError(format!("Failed to get model info: {}", e)))
    }
}
