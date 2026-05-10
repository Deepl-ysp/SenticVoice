use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub pitch_offset: Option<f32>,
    pub speaking_rate: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub custom_dims: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsodyControl {
    pub pitch_offset: f32,
    pub pitch_range: f32,
    pub pitch_variation: f32,
    pub speaking_rate: f32,
    pub pause_duration: f32,
    pub energy_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCharacteristics {
    pub breathiness: f32,
    pub roughness: f32,
    pub creakiness: f32,
    pub formant_shift: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMODData {
    pub version: String,
    pub metadata: EMODMetadata,
    pub emotion_vector: EmotionVector,
    pub prosody_control: ProsodyControl,
    pub style_markers: Vec<StyleMarker>,
    pub voice_characteristics: VoiceCharacteristics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMODMetadata {
    pub name: String,
    pub description: String,
    pub author: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleMarker {
    #[serde(rename = "type")]
    pub marker_type: String,
    pub pitch_curve: String,
    pub duration_factor: f32,
}

pub struct EmotionService {
    emotions_dir: PathBuf,
    presets: HashMap<String, EmotionPreset>,
}

impl EmotionService {
    pub fn new(emotions_dir: &PathBuf) -> Self {
        let mut presets = HashMap::new();
        
        presets.insert("neutral".to_string(), EmotionPreset {
            id: "neutral".to_string(),
            name: "中性".to_string(),
            description: "中性、平静的朗读风格".to_string(),
            valence: 0.0,
            arousal: 0.0,
            dominance: 0.0,
            pitch_offset: Some(0.0),
            speaking_rate: Some(1.0),
        });
        
        presets.insert("happy".to_string(), EmotionPreset {
            id: "happy".to_string(),
            name: "快乐".to_string(),
            description: "快乐、积极的情绪".to_string(),
            valence: 0.8,
            arousal: 0.6,
            dominance: 0.3,
            pitch_offset: Some(3.0),
            speaking_rate: Some(1.1),
        });
        
        presets.insert("sad".to_string(), EmotionPreset {
            id: "sad".to_string(),
            name: "悲伤".to_string(),
            description: "悲伤、低沉的情绪".to_string(),
            valence: -0.7,
            arousal: -0.3,
            dominance: -0.4,
            pitch_offset: Some(-2.0),
            speaking_rate: Some(0.85),
        });
        
        presets.insert("angry".to_string(), EmotionPreset {
            id: "angry".to_string(),
            name: "愤怒".to_string(),
            description: "愤怒、激动的情绪".to_string(),
            valence: -0.5,
            arousal: 0.8,
            dominance: 0.6,
            pitch_offset: Some(4.0),
            speaking_rate: Some(1.2),
        });
        
        presets.insert("calm".to_string(), EmotionPreset {
            id: "calm".to_string(),
            name: "平静".to_string(),
            description: "平静、放松的情绪".to_string(),
            valence: 0.3,
            arousal: -0.4,
            dominance: 0.1,
            pitch_offset: Some(-1.0),
            speaking_rate: Some(0.9),
        });
        
        presets.insert("excited".to_string(), EmotionPreset {
            id: "excited".to_string(),
            name: "兴奋".to_string(),
            description: "兴奋、热情的情绪".to_string(),
            valence: 0.7,
            arousal: 0.9,
            dominance: 0.5,
            pitch_offset: Some(5.0),
            speaking_rate: Some(1.15),
        });
        
        presets.insert("storytelling".to_string(), EmotionPreset {
            id: "storytelling".to_string(),
            name: "讲故事".to_string(),
            description: "讲故事风格，富有表现力".to_string(),
            valence: 0.4,
            arousal: 0.2,
            dominance: 0.3,
            pitch_offset: Some(0.0),
            speaking_rate: Some(1.0),
        });

        Self {
            emotions_dir: emotions_dir.clone(),
            presets,
        }
    }

    pub fn list_presets(&self) -> Vec<EmotionPreset> {
        self.presets.values().cloned().collect()
    }

    pub fn get_preset(&self, preset_id: &str) -> Result<EmotionPreset> {
        self.presets
            .get(preset_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Preset not found: {}", preset_id)))
    }

    pub fn create_emod(&self, name: &str, preset_id: Option<&str>) -> Result<EMODData> {
        let emotion_vector = if let Some(pid) = preset_id {
            let preset = self.get_preset(pid)?;
            EmotionVector {
                valence: preset.valence,
                arousal: preset.arousal,
                dominance: preset.dominance,
                custom_dims: HashMap::new(),
            }
        } else {
            EmotionVector {
                valence: 0.0,
                arousal: 0.0,
                dominance: 0.0,
                custom_dims: HashMap::new(),
            }
        };

        let prosody_control = ProsodyControl {
            pitch_offset: 0.0,
            pitch_range: 1.0,
            pitch_variation: 0.5,
            speaking_rate: 1.0,
            pause_duration: 1.0,
            energy_level: 0.6,
        };

        let voice_characteristics = VoiceCharacteristics {
            breathiness: 0.1,
            roughness: 0.1,
            creakiness: 0.05,
            formant_shift: 0.0,
        };

        Ok(EMODData {
            version: "1.0".to_string(),
            metadata: EMODMetadata {
                name: name.to_string(),
                description: format!("Auto-generated emotion: {}", name),
                author: "user".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                tags: vec![],
            },
            emotion_vector,
            prosody_control,
            style_markers: vec![
                StyleMarker {
                    marker_type: "sentence_start".to_string(),
                    pitch_curve: "rise".to_string(),
                    duration_factor: 1.1,
                },
                StyleMarker {
                    marker_type: "sentence_end".to_string(),
                    pitch_curve: "fall".to_string(),
                    duration_factor: 1.2,
                },
                StyleMarker {
                    marker_type: "question".to_string(),
                    pitch_curve: "rise_sharp".to_string(),
                    duration_factor: 1.0,
                },
            ],
            voice_characteristics,
        })
    }

    pub fn save_emod(&self, emod: &EMODData, filename: &str) -> Result<String> {
        let path = self.emotions_dir.join(format!("{}.emod", filename));
        
        fs::create_dir_all(&self.emotions_dir)?;
        
        let json = serde_json::to_string_pretty(emod)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize EMOD: {}", e)))?;
        
        fs::write(&path, json)?;
        
        Ok(path.to_string_lossy().to_string())
    }

    pub fn load_emod(&self, filename: &str) -> Result<EMODData> {
        let path = self.emotions_dir.join(format!("{}.emod", filename));
        
        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::IoError(format!("Failed to read EMOD file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| AppError::ValidationError(format!("Failed to parse EMOD: {}", e)))
    }

    pub fn emotion_vector_to_emod(&self, vector: &EmotionVector) -> EMODData {
        let pitch_offset = if vector.valence > 0.0 {
            vector.valence * 5.0
        } else {
            vector.valence * 2.0
        };
        
        let speaking_rate = 1.0 + (vector.arousal * 0.2);

        EMODData {
            version: "1.0".to_string(),
            metadata: EMODMetadata {
                name: "Custom Emotion".to_string(),
                description: "Custom emotion from vector".to_string(),
                author: "user".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                tags: vec![],
            },
            emotion_vector: vector.clone(),
            prosody_control: ProsodyControl {
                pitch_offset,
                pitch_range: 1.0 + (vector.arousal.abs() * 0.3),
                pitch_variation: 0.5 + (vector.dominance.abs() * 0.3),
                speaking_rate: speaking_rate.max(0.5).min(1.5),
                pause_duration: 1.0,
                energy_level: 0.6 + (vector.arousal * 0.2),
            },
            style_markers: vec![],
            voice_characteristics: VoiceCharacteristics {
                breathiness: 0.1,
                roughness: 0.1,
                creakiness: 0.05,
                formant_shift: 0.0,
            },
        }
    }
}
