use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalError(String),
    NotFound(String),
    BadRequest(String),
    ValidationError(String),
    IoError(String),
    PythonError(String),
    ModelError(String),
    SynthesisError(String),
    TrainingError(String),
    AnnotationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::IoError(msg) => write!(f, "IO Error: {}", msg),
            AppError::PythonError(msg) => write!(f, "Python Error: {}", msg),
            AppError::ModelError(msg) => write!(f, "Model Error: {}", msg),
            AppError::SynthesisError(msg) => write!(f, "Synthesis Error: {}", msg),
            AppError::TrainingError(msg) => write!(f, "Training Error: {}", msg),
            AppError::AnnotationError(msg) => write!(f, "Annotation Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = serde_json::json!({
            "success": false,
            "error": {
                "code": self.error_code(),
                "message": self.to_string()
            }
        });

        match self {
            AppError::InternalError(_) | AppError::PythonError(_) | AppError::IoError(_) => {
                HttpResponse::InternalServerError().json(error_response)
            }
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::BadRequest(_) | AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(error_response)
            }
            AppError::ModelError(_) | AppError::SynthesisError(_) | AppError::TrainingError(_) | AppError::AnnotationError(_) => {
                HttpResponse::BadRequest().json(error_response)
            }
        }
    }
}

impl AppError {
    fn error_code(&self) -> &str {
        match self {
            AppError::InternalError(_) => "INTERNAL_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::IoError(_) => "IO_ERROR",
            AppError::PythonError(_) => "PYTHON_ERROR",
            AppError::ModelError(_) => "MODEL_ERROR",
            AppError::SynthesisError(_) => "SYNTHESIS_ERROR",
            AppError::TrainingError(_) => "TRAINING_ERROR",
            AppError::AnnotationError(_) => "ANNOTATION_ERROR",
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<pyo3::PyErr> for AppError {
    fn from(err: pyo3::PyErr) -> Self {
        AppError::PythonError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
