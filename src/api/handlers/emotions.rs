use actix_web::{web, HttpResponse, Result};
use crate::services::emotion_service::{EmotionService, EmotionPreset};
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateEmodRequest {
    pub name: String,
    pub preset_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmodResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub async fn list_presets(
    service: web::Data<EmotionService>,
) -> Result<HttpResponse> {
    let presets = service.list_presets();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "presets": presets }
    })))
}

pub async fn get_preset(
    service: web::Data<EmotionService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let preset_id = path.into_inner();
    
    match service.get_preset(&preset_id) {
        Ok(preset) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": preset
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

pub async fn create_emod(
    service: web::Data<EmotionService>,
    body: web::Json<CreateEmodRequest>,
) -> Result<HttpResponse> {
    let request = body.into_inner();
    
    match service.create_emod(&request.name, request.preset_id.as_deref()) {
        Ok(emod) => {
            let filename = format!("emod_{}", uuid::Uuid::new_v4());
            match service.save_emod(&emod, &filename) {
                Ok(path) => Ok(HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "data": {
                        "emod": emod,
                        "path": path
                    }
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": e.to_string()
                }))),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn get_emod(
    service: web::Data<EmotionService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let filename = path.into_inner();
    
    match service.load_emod(&filename) {
        Ok(emod) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": emod
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
