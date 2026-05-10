use actix_web::{web, HttpResponse, Result};
use crate::services::training_service::{TrainingService, TrainingRequest};
use crate::error::AppError;

pub async fn start_training(
    service: web::Data<TrainingService>,
    body: web::Json<TrainingRequest>,
) -> Result<HttpResponse> {
    let request = body.into_inner();
    
    match service.start_training(request).await {
        Ok(job) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": {
                "training_id": job.id,
                "status": job.status,
                "estimated_time": job.estimated_time
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn get_training_status(
    service: web::Data<TrainingService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let training_id = path.into_inner();
    
    match service.get_status(&training_id).await {
        Ok(status) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": status
        }))),
        Err(e) => {
            let status_code = if matches!(e, AppError::NotFound(_)) {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            Ok(HttpResponse::build(status_code).json(serde_json::json!({
                "success": false,
                "error": e.to_string()
            })))
        }
    }
}

pub async fn cancel_training(
    service: web::Data<TrainingService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let training_id = path.into_inner();
    
    match service.cancel_training(&training_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "message": "Training cancelled successfully" }
        }))),
        Err(e) => {
            let status_code = if matches!(e, AppError::NotFound(_)) {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
            };
            Ok(HttpResponse::build(status_code).json(serde_json::json!({
                "success": false,
                "error": e.to_string()
            })))
        }
    }
}

pub async fn list_trainings(
    service: web::Data<TrainingService>,
) -> Result<HttpResponse> {
    match service.list_jobs().await {
        Ok(jobs) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "trainings": jobs }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}
