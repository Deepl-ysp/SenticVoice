use actix_web::{web, Scope};
use crate::api::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/models")
                    .route("", web::get().to(handlers::models::list_models))
                    .route("", web::post().to(handlers::models::upload_model))
                    .route("/{id}", web::get().to(handlers::models::get_model))
                    .route("/{id}", web::delete().to(handlers::models::delete_model))
            )
            .service(
                web::scope("/tts")
                    .route("/synthesize", web::post().to(handlers::tts::synthesize))
                    .route("/preload/{model_path}", web::post().to(handlers::tts::preload_model))
                    .route("/model-info/{model_path}", web::get().to(handlers::tts::get_model_info))
            )
            .service(
                web::scope("/training")
                    .route("", web::get().to(handlers::training::list_trainings))
                    .route("/start", web::post().to(handlers::training::start_training))
                    .route("/status/{id}", web::get().to(handlers::training::get_training_status))
                    .route("/cancel/{id}", web::post().to(handlers::training::cancel_training))
            )
            .service(
                web::scope("/annotations")
                    .route("", web::post().to(handlers::annotations::create_annotation))
                    .route("/transcribe", web::post().to(handlers::annotations::transcribe))
                    .route("/{id}", web::put().to(handlers::annotations::update_annotation))
                    .route("/export", web::post().to(handlers::annotations::export_annotations))
            )
            .service(
                web::scope("/emotions")
                    .route("/presets", web::get().to(handlers::emotions::list_presets))
                    .route("/presets/{id}", web::get().to(handlers::emotions::get_preset))
                    .route("", web::post().to(handlers::emotions::create_emod))
                    .route("/{filename}", web::get().to(handlers::emotions::get_emod))
            )
    );
}
