use actix_web::{web, HttpResponse, Result};
use crate::services::ModelService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UploadModelRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ModelListResponse {
    pub success: bool,
    pub data: Option<crate::services::model_service::ModelListResponse>,
    pub error: Option<String>,
}

pub async fn list_models(
    service: web::Data<ModelService>,
) -> Result<HttpResponse> {
    match service.list_models() {
        Ok(models) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": models
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn get_model(
    service: web::Data<ModelService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let model_id = path.into_inner();
    
    match service.get_model(&model_id) {
        Ok(model) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": model
        }))),
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn upload_model(
    service: web::Data<ModelService>,
    body: web::Json<UploadModelRequest>,
) -> Result<HttpResponse> {
    let model_data = b"dummy_model_data".to_vec();
    
    match service.upload_model(&body.name, &body.description, &model_data) {
        Ok(model) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": model
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn delete_model(
    service: web::Data<ModelService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let model_id = path.into_inner();
    
    match service.delete_model(&model_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "message": "Model deleted successfully" }
        }))),
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}
