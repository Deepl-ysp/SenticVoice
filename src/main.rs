use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;

mod api;
mod config;
mod error;
mod services;

use crate::config::AppConfig;
use crate::services::{ModelService, EmotionService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let config = AppConfig::from_env();
    
    std::fs::create_dir_all(&config.models_dir)?;
    std::fs::create_dir_all(&config.output_dir)?;
    std::fs::create_dir_all(&config.training_dir)?;
    std::fs::create_dir_all(&config.data_dir)?;

    let model_service = Arc::new(ModelService::new(&config.models_dir));
    let emotion_service = Arc::new(EmotionService::new(&config.data_dir));

    let server_addr = config.server_addr();
    println!("AI TTS Backend Server starting on http://{}", server_addr);
    println!("Models directory: {:?}", config.models_dir);
    println!("Data directory: {:?}", config.data_dir);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600)
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(model_service.clone()))
            .app_data(web::Data::new(emotion_service.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(api::routes::configure)
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
    })
    .bind(&server_addr)?
    .run()
    .await
}

async fn index() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "name": "AI TTS Backend Server",
        "version": "0.1.0",
        "status": "running"
    }))
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
