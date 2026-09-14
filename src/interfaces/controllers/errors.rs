use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    ValidationError(String),
    NotFound(String),
    DuplicateResource(String),
    Unauthorized(String),
    InternalError(String),
    DatabaseError(String),
    RepositoryError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
            ApiError::DuplicateResource(msg) => write!(f, "Recurso duplicado: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "No autorizado: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Error interno: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Error de base de datos: {}", msg),
            ApiError::RepositoryError(msg) => write!(f, "Error de repositorio: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(_) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: self.to_string(),
                    status: 400,
                })
            }
            ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "NOT_FOUND".to_string(),
                    message: self.to_string(),
                    status: 404,
                })
            }
            ApiError::DuplicateResource(_) => {
                HttpResponse::Conflict().json(ErrorResponse {
                    error: "DUPLICATE_RESOURCE".to_string(),
                    message: self.to_string(),
                    status: 409,
                })
            }
            ApiError::Unauthorized(_) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "UNAUTHORIZED".to_string(),
                    message: self.to_string(),
                    status: 401,
                })
            }
            ApiError::InternalError(_) | ApiError::DatabaseError(_) | ApiError::RepositoryError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "INTERNAL_ERROR".to_string(),
                    message: self.to_string(),
                    status: 500,
                })
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, status: u16) -> Self {
        ErrorResponse {
            error,
            message,
            status,
        }
    }
    
    pub fn validation(message: String) -> Self {
        ErrorResponse::new("VALIDATION_ERROR".to_string(), message, 400)
    }
    
    pub fn not_found(message: String) -> Self {
        ErrorResponse::new("NOT_FOUND".to_string(), message, 404)
    }
    
    pub fn duplicate(message: String) -> Self {
        ErrorResponse::new("DUPLICATE_RESOURCE".to_string(), message, 409)
    }
    
    pub fn internal(message: String) -> Self {
        ErrorResponse::new("INTERNAL_ERROR".to_string(), message, 500)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Duplicate(String),
    ConnectionError(String),
    QueryError(String),
    TransactionError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
            RepositoryError::Duplicate(msg) => write!(f, "Recurso duplicado: {}", msg),
            RepositoryError::ConnectionError(msg) => write!(f, "Error de conexión: {}", msg),
            RepositoryError::QueryError(msg) => write!(f, "Error de consulta: {}", msg),
            RepositoryError::TransactionError(msg) => write!(f, "Error de transacción: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<diesel::result::Error> for RepositoryError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                RepositoryError::NotFound("Registro no encontrado".to_string())
            }
            diesel::result::Error::DatabaseError(_, info) => {
                RepositoryError::QueryError(info.message().to_string())
            }
            _ => RepositoryError::QueryError(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_response_validation() {
        let response = ErrorResponse::validation("Campo inválido".to_string());
        
        assert_eq!(response.error, "VALIDATION_ERROR");
        assert_eq!(response.status, 400);
    }
    
    #[test]
    fn test_error_response_not_found() {
        let response = ErrorResponse::not_found("Recurso no encontrado".to_string());
        
        assert_eq!(response.error, "NOT_FOUND");
        assert_eq!(response.status, 404);
    }
    
    #[test]
    fn test_error_response_duplicate() {
        let response = ErrorResponse::duplicate("Recurso duplicado".to_string());
        
        assert_eq!(response.error, "DUPLICATE_RESOURCE");
        assert_eq!(response.status, 409);
    }
    
    #[test]
    fn test_error_response_internal() {
        let response = ErrorResponse::internal("Error interno".to_string());
        
        assert_eq!(response.error, "INTERNAL_ERROR");
        assert_eq!(response.status, 500);
    }
    
    #[test]
    fn test_api_error_display() {
        let error = ApiError::ValidationError("Campo requerido".to_string());
        let display = format!("{}", error);
        
        assert!(display.contains("Error de validación"));
        assert!(display.contains("Campo requerido"));
    }
    
    #[test]
    fn test_repository_error_display() {
        let error = RepositoryError::NotFound("ID no encontrado".to_string());
        let display = format!("{}", error);
        
        assert!(display.contains("Recurso no encontrado"));
    }
}