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
                web::scope("/emotions")
                    .route("/presets", web::get().to(handlers::emotions::list_presets))
                    .route("/presets/{id}", web::get().to(handlers::emotions::get_preset))
            )
    );
}
