use crate::error::{AppError, Result};
use crate::python_bridge::annotator_bridge::{AnnotatorBridge, TranscriptionResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub audio_file: String,
    pub text: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationSegment {
    pub id: u32,
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: f64,
    pub words: Vec<WordTimestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordTimestamp {
    pub word: String,
    pub start: f64,
    pub end: f64,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub transcription_id: String,
    pub text: String,
    pub segments: Vec<AnnotationSegment>,
    pub language: String,
    pub duration: f64,
}

pub struct AnnotationService {
    bridge: Arc<RwLock<Option<AnnotatorBridge>>>,
    data_dir: PathBuf,
}

impl AnnotationService {
    pub fn new(data_dir: &PathBuf) -> Self {
        Self {
            bridge: Arc::new(RwLock::new(None)),
            data_dir: data_dir.clone(),
        }
    }

    pub async fn initialize_bridge(&self) -> Result<()> {
        let bridge = AnnotatorBridge::new()
            .map_err(|e| AppError::PythonError(format!("Failed to initialize annotator bridge: {}", e)))?;
        
        let mut bridge_lock = self.bridge.write().await;
        *bridge_lock = Some(bridge);
        
        Ok(())
    }

    pub async fn transcribe(&self, audio_path: &str, language: &str) -> Result<TranscriptionResponse> {
        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("Annotator bridge not initialized".to_string()))?;

        let result = bridge.transcribe(audio_path, language)
            .map_err(|e| AppError::AnnotationError(format!("Transcription failed: {}", e)))?;

        let transcription_id = Uuid::new_v4().to_string();
        let duration = result.segments.last()
            .map(|s| s.end)
            .unwrap_or(0.0);

        let segments: Vec<AnnotationSegment> = result.segments
            .into_iter()
            .enumerate()
            .map(|(i, s)| AnnotationSegment {
                id: i as u32,
                start: s.start,
                end: s.end,
                text: s.text,
                confidence: s.confidence,
                words: Vec::new(),
            })
            .collect();

        Ok(TranscriptionResponse {
            transcription_id,
            text: result.text,
            segments,
            language: language.to_string(),
            duration,
        })
    }

    pub async fn batch_transcribe(
        &self,
        audio_paths: Vec<String>,
        language: &str,
    ) -> Result<Vec<TranscriptionResponse>> {
        let bridge_lock = self.bridge.read().await;
        let bridge = bridge_lock.as_ref()
            .ok_or_else(|| AppError::InternalError("Annotator bridge not initialized".to_string()))?;

        let results = bridge.batch_transcribe(audio_paths, language)
            .map_err(|e| AppError::AnnotationError(format!("Batch transcription failed: {}", e)))?;

        let responses: Vec<TranscriptionResponse> = results
            .into_iter()
            .map(|r| {
                let transcription_id = Uuid::new_v4().to_string();
                let duration = r.segments.last().map(|s| s.end).unwrap_or(0.0);
                let segments: Vec<AnnotationSegment> = r.segments
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| AnnotationSegment {
                        id: i as u32,
                        start: s.start,
                        end: s.end,
                        text: s.text,
                        confidence: s.confidence,
                        words: Vec::new(),
                    })
                    .collect();

                TranscriptionResponse {
                    transcription_id,
                    text: r.text,
                    segments,
                    language: language.to_string(),
                    duration,
                }
            })
            .collect();

        Ok(responses)
    }

    pub fn create_annotation(&self, audio_file: &str, text: &str) -> Result<Annotation> {
        Ok(Annotation {
            id: Uuid::new_v4().to_string(),
            audio_file: audio_file.to_string(),
            text: text.to_string(),
            start_time: None,
            end_time: None,
            verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn update_annotation(
        &self,
        annotation: &mut Annotation,
        text: Option<&str>,
        start_time: Option<f64>,
        end_time: Option<f64>,
        verified: Option<bool>,
    ) -> Result<()> {
        if let Some(t) = text {
            annotation.text = t.to_string();
        }
        if let Some(st) = start_time {
            annotation.start_time = Some(st);
        }
        if let Some(et) = end_time {
            annotation.end_time = Some(et);
        }
        if let Some(v) = verified {
            annotation.verified = v;
        }
        annotation.updated_at = Utc::now();
        Ok(())
    }

    pub fn export_annotations(&self, annotations: &[Annotation]) -> Result<String> {
        let export_data: Vec<serde_json::Value> = annotations
            .iter()
            .filter(|a| a.verified)
            .map(|a| {
                serde_json::json!({
                    "audio_file": a.audio_file,
                    "text": a.text,
                    "duration": a.end_time.zip(a.start_time).map(|(e, s)| e - s),
                })
            })
            .collect();

        let jsonl: String = export_data
            .iter()
            .map(|item| serde_json::to_string(item).unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(jsonl)
    }
}
