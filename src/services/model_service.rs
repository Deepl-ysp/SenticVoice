use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub model_path: String,
    pub index_path: Option<String>,
    pub emod_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub file_size: u64,
    pub metadata: ModelMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub sample_rate: u32,
    pub language: String,
    pub duration: Option<f64>,
    pub is_loaded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub models: Vec<VoiceModel>,
    pub total: usize,
}

pub struct ModelService {
    models_dir: PathBuf,
}

impl ModelService {
    pub fn new(models_dir: &Path) -> Self {
        Self {
            models_dir: models_dir.to_path_buf(),
        }
    }

    pub fn list_models(&self) -> Result<ModelListResponse> {
        let mut models = Vec::new();
        
        if !self.models_dir.exists() {
            fs::create_dir_all(&self.models_dir)?;
        }

        for entry in fs::read_dir(&self.models_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(model) = self.load_model_from_dir(&path)? {
                    models.push(model);
                }
            }
        }

        let total = models.len();
        Ok(ModelListResponse { models, total })
    }

    fn load_model_from_dir(&self, dir: &Path) -> Result<Option<VoiceModel>> {
        let model_file = dir.join("model.pth");
        if !model_file.exists() {
            return Ok(None);
        }

        let config_file = dir.join("config.json");
        let metadata = if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        let file_size = fs::metadata(&model_file)?.len();
        let model_name = dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let emod_path = dir.join("emotion.emod");
        let index_path = dir.join("features.index");

        Ok(Some(VoiceModel {
            id: Uuid::new_v4().to_string(),
            name: metadata.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&model_name)
                .to_string(),
            description: metadata.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            model_path: model_file.to_string_lossy().to_string(),
            index_path: if index_path.exists() { Some(index_path.to_string_lossy().to_string()) } else { None },
            emod_path: if emod_path.exists() { Some(emod_path.to_string_lossy().to_string()) } else { None },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            file_size,
            metadata: ModelMetadata {
                sample_rate: metadata.get("sample_rate").and_then(|v| v.as_u64()).unwrap_or(22050) as u32,
                language: metadata.get("language").and_then(|v| v.as_str()).unwrap_or("zh").to_string(),
                duration: metadata.get("duration").and_then(|v| v.as_f64()),
                is_loaded: false,
            },
        }))
    }

    pub fn get_model(&self, model_id: &str) -> Result<VoiceModel> {
        let models = self.list_models()?.models;
        
        models.into_iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| AppError::NotFound(format!("Model not found: {}", model_id)))
    }

    pub fn upload_model(&self, name: &str, description: &str, model_data: &[u8]) -> Result<VoiceModel> {
        let model_id = Uuid::new_v4().to_string();
        let model_dir = self.models_dir.join(&model_id);
        
        fs::create_dir_all(&model_dir)?;

        let model_path = model_dir.join("model.pth");
        fs::write(&model_path, model_data)?;

        let config = serde_json::json!({
            "name": name,
            "description": description,
            "created_at": Utc::now().to_rfc3339(),
        });
        
        let config_path = model_dir.join("config.json");
        fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

        self.load_model_from_dir(&model_dir)?
            .ok_or_else(|| AppError::InternalError("Failed to load uploaded model".to_string()))
    }

    pub fn delete_model(&self, model_id: &str) -> Result<()> {
        let model_dir = self.models_dir.join(model_id);
        
        if !model_dir.exists() {
            return Err(AppError::NotFound(format!("Model not found: {}", model_id)));
        }

        fs::remove_dir_all(&model_dir)?;
        Ok(())
    }

    pub fn get_model_path(&self, model_id: &str) -> Result<PathBuf> {
        let model = self.get_model(model_id)?;
        Ok(PathBuf::from(model.model_path))
    }

    pub fn save_emod(&self, model_id: &str, emod_data: &[u8]) -> Result<String> {
        let model_dir = self.models_dir.join(model_id);
        
        if !model_dir.exists() {
            return Err(AppError::NotFound(format!("Model not found: {}", model_id)));
        }

        let emod_path = model_dir.join("emotion.emod");
        fs::write(&emod_path, emod_data)?;

        Ok(emod_path.to_string_lossy().to_string())
    }

    pub fn load_emod(&self, model_id: &str) -> Result<String> {
        let model = self.get_model(model_id)?;
        
        let emod_path = model.emod_path
            .ok_or_else(|| AppError::NotFound("EMOD file not found".to_string()))?;

        fs::read_to_string(&emod_path)
            .map_err(|e| AppError::IoError(format!("Failed to read EMOD file: {}", e)))
    }
}
