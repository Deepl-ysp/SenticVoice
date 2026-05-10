use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRequest {
    pub model_name: String,
    pub training_data: TrainingData,
    pub config: TrainingConfig,
    pub base_model: Option<String>,
    pub emotion_extraction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub audio_files: Vec<String>,
    pub annotations_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f32,
    pub save_every: u32,
    pub validation_split: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub id: String,
    pub name: String,
    pub status: TrainingStatus,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub output_model_path: Option<String>,
    pub error_message: Option<String>,
    pub estimated_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrainingStatus {
    Pending,
    Preparing,
    Training,
    Validating,
    Exporting,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingStatusResponse {
    pub training_id: String,
    pub status: TrainingStatus,
    pub progress: f32,
    pub current_epoch: Option<u32>,
    pub total_epochs: u32,
    pub loss: Option<f32>,
    pub validation_loss: Option<f32>,
    pub estimated_time_remaining: Option<u64>,
}

pub struct TrainingService {
    training_dir: PathBuf,
    jobs: Arc<RwLock<std::collections::HashMap<String, TrainingJob>>>,
}

impl TrainingService {
    pub fn new(training_dir: &PathBuf) -> Self {
        Self {
            training_dir: training_dir.clone(),
            jobs: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn start_training(&self, request: TrainingRequest) -> Result<TrainingJob> {
        let job_id = Uuid::new_v4().to_string();
        let job_dir = self.training_dir.join(&job_id);
        
        std::fs::create_dir_all(&job_dir)
            .map_err(|e| AppError::IoError(format!("Failed to create job directory: {}", e)))?;

        let job = TrainingJob {
            id: job_id.clone(),
            name: request.model_name,
            status: TrainingStatus::Pending,
            progress: 0.0,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            output_model_path: None,
            error_message: None,
            estimated_time: Some(self.estimate_training_time(&request.config)),
        };

        {
            let mut jobs = self.jobs.write().await;
            jobs.insert(job_id.clone(), job.clone());
        }

        tokio::spawn({
            let jobs = self.jobs.clone();
            let job_id = job_id.clone();
            let job_dir = job_dir.clone();
            let request = request.clone();

            async move {
                Self::run_training_job(jobs, job_id, job_dir, request).await;
            }
        });

        Ok(job)
    }

    async fn run_training_job(
        jobs: Arc<RwLock<std::collections::HashMap<String, TrainingJob>>>,
        job_id: String,
        job_dir: PathBuf,
        request: TrainingRequest,
    ) {
        {
            let mut jobs_guard = jobs.write().await;
            if let Some(job) = jobs_guard.get_mut(&job_id) {
                job.status = TrainingStatus::Preparing;
                job.started_at = Some(Utc::now());
            }
        }

        {
            let mut jobs_guard = jobs.write().await;
            if let Some(job) = jobs_guard.get_mut(&job_id) {
                job.status = TrainingStatus::Training;
            }
        }

        let total_epochs = request.config.epochs;
        
        for epoch in 1..=total_epochs {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            let progress = (epoch as f32 / total_epochs as f32) * 100.0;
            
            {
                let mut jobs_guard = jobs.write().await;
                if let Some(job) = jobs_guard.get_mut(&job_id) {
                    job.progress = progress;
                }
            }

            if epoch == total_epochs {
                {
                    let mut jobs_guard = jobs.write().await;
                    if let Some(job) = jobs_guard.get_mut(&job_id) {
                        job.status = TrainingStatus::Exporting;
                    }
                }

                let model_path = job_dir.join("model.pth");
                let _ = std::fs::write(&model_path, b"dummy_model_data");
                
                {
                    let mut jobs_guard = jobs.write().await;
                    if let Some(job) = jobs_guard.get_mut(&job_id) {
                        job.status = TrainingStatus::Completed;
                        job.completed_at = Some(Utc::now());
                        job.progress = 100.0;
                        job.output_model_path = Some(model_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    pub async fn get_status(&self, job_id: &str) -> Result<TrainingStatusResponse> {
        let jobs = self.jobs.read().await;
        
        let job = jobs.get(job_id)
            .ok_or_else(|| AppError::NotFound(format!("Training job not found: {}", job_id)))?;

        Ok(TrainingStatusResponse {
            training_id: job.id.clone(),
            status: job.status.clone(),
            progress: job.progress,
            current_epoch: Some((job.progress / 100.0 * 1000.0) as u32),
            total_epochs: 1000,
            loss: Some(0.5),
            validation_loss: Some(0.3),
            estimated_time_remaining: job.estimated_time,
        })
    }

    pub async fn cancel_training(&self, job_id: &str) -> Result<()> {
        let mut jobs = self.jobs.write().await;
        
        let job = jobs.get_mut(job_id)
            .ok_or_else(|| AppError::NotFound(format!("Training job not found: {}", job_id)))?;

        if job.status == TrainingStatus::Completed || job.status == TrainingStatus::Failed {
            return Err(AppError::BadRequest(format!("Cannot cancel job in {} state", job.status)));
        }

        job.status = TrainingStatus::Cancelled;
        job.completed_at = Some(Utc::now());
        
        Ok(())
    }

    pub async fn list_jobs(&self) -> Result<Vec<TrainingJob>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.values().cloned().collect())
    }

    fn estimate_training_time(&self, config: &TrainingConfig) -> u64 {
        let base_time = config.epochs as u64 * 10;
        let batch_factor = (16.0 / config.batch_size as f32).ceil() as u64;
        base_time * batch_factor
    }
}
