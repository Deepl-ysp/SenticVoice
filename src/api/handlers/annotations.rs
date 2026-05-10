use actix_web::{web, HttpResponse, Result};
use crate::services::annotation_service::{AnnotationService, Annotation};
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TranscribeRequest {
    pub audio_path: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnnotationRequest {
    pub audio_file: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnnotationRequest {
    pub text: Option<String>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub verified: Option<bool>,
}

pub async fn transcribe(
    service: web::Data<AnnotationService>,
    body: web::Json<TranscribeRequest>,
) -> Result<HttpResponse> {
    let request = body.into_inner();
    let language = request.language.unwrap_or_else(|| "zh".to_string());
    
    match service.transcribe(&request.audio_path, &language).await {
        Ok(result) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": result
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn create_annotation(
    service: web::Data<AnnotationService>,
    body: web::Json<CreateAnnotationRequest>,
) -> Result<HttpResponse> {
    let request = body.into_inner();
    
    match service.create_annotation(&request.audio_file, &request.text) {
        Ok(annotation) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": annotation
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn update_annotation(
    service: web::Data<AnnotationService>,
    path: web::Path<String>,
    body: web::Json<UpdateAnnotationRequest>,
) -> Result<HttpResponse> {
    let annotation_id = path.into_inner();
    let request = body.into_inner();
    
    let mut annotation = Annotation {
        id: annotation_id,
        audio_file: String::new(),
        text: String::new(),
        start_time: None,
        end_time: None,
        verified: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    match service.update_annotation(
        &mut annotation,
        request.text.as_deref(),
        request.start_time,
        request.end_time,
        request.verified,
    ) {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": annotation
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn export_annotations(
    service: web::Data<AnnotationService>,
) -> Result<HttpResponse> {
    let annotations: Vec<Annotation> = Vec::new();
    
    match service.export_annotations(&annotations) {
        Ok(jsonl) => Ok(HttpResponse::Ok()
            .content_type("application/jsonl")
            .body(jsonl)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}
