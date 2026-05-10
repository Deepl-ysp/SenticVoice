use actix_web::{web, HttpResponse, Result};
use crate::services::tts_service::{TTSService, SynthesizeRequest};
use crate::error::AppError;

pub async fn synthesize(
    service: web::Data<TTSService>,
    body: web::Json<SynthesizeRequest>,
) -> Result<HttpResponse> {
    let request = body.into_inner();
    
    match service.synthesize(request).await {
        Ok(response) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": response
        }))),
        Err(e) => {
            let (status, code) = match &e {
                AppError::ValidationError(_) => (400, "VALIDATION_ERROR"),
                AppError::SynthesisError(_) => (400, "SYNTHESIS_ERROR"),
                _ => (500, "INTERNAL_ERROR"),
            };
            Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
                .json(serde_json::json!({
                    "success": false,
                    "error": {
                        "code": code,
                        "message": e.to_string()
                    }
                })))
        }
    }
}

pub async fn preload_model(
    service: web::Data<TTSService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let model_path = path.into_inner();
    
    match service.preload_model(&model_path).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "message": "Model preloaded successfully" }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn get_model_info(
    service: web::Data<TTSService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let model_path = path.into_inner();
    
    match service.get_model_info(&model_path).await {
        Ok(info) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": info
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}
