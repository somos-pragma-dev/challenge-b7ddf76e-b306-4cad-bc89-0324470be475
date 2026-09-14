use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::loan_request::LoanRequest;
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, UpdateLoanStatusDto, LoanListResponseDto};
use crate::interfaces::controllers::errors::{ApiError, ApiResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct PathParams {
    pub id: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<String> {
        success: true,
        data: Some("API de gestión de préstamos operativa".to_string()),
        message: None,
    })
}

pub async fn create_loan(
    pool: web::Data<crate::config::database::DbPool>,
    dto: web::Json<CreateLoanRequestDto>,
) -> Result<impl Responder, ApiError> {
    dto.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let loan_request = LoanRequest::new(
        dto.applicant_name.clone(),
        dto.amount,
        dto.term_months,
        dto.request_id.clone(),
    ).map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let repository = pool.get_repository();
    
    let exists = repository.exists_by_request_id(&dto.request_id).await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    if exists {
        return Err(ApiError::DuplicateResource(
            "Ya existe una solicitud con este identificador".to_string()
        ));
    }
    
    let created = repository.create(loan_request).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    let response = LoanResponseDto::from(created);
    
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Solicitud de préstamo creada exitosamente".to_string()),
    }))
}

pub async fn get_loan(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    
    let repository = pool.get_repository();
    
    let loan = repository.find_by_id(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    match loan {
        Some(req) => {
            let response = LoanResponseDto::from(req);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            }))
        }
        None => Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        )),
    }
}

pub async fn get_all_loans(
    pool: web::Data<crate::config::database::DbPool>,
) -> Result<impl Responder, ApiError> {
    let repository = pool.get_repository();
    
    let loans = repository.find_all().await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    let responses: Vec<LoanResponseDto> = loans.into_iter()
        .map(LoanResponseDto::from)
        .collect();
    
    let total = responses.len();
    
    Ok(HttpResponse::Ok().json(LoanListResponseDto {
        success: true,
        data: responses,
        total,
        message: Some("Lista de solicitudes de préstamo".to_string()),
    }))
}

pub async fn update_loan_status(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
    dto: web::Json<UpdateLoanStatusDto>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    dto.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let repository = pool.get_repository();
    
    let mut loan = repository.find_by_id(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    match loan {
        Some(mut req) => {
            let new_status = crate::domain::models::loan_request::LoanStatus::from(dto.status.clone());
            req.update_status(new_status);
            
            let updated = repository.update(req).await
                .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
            
            let response = LoanResponseDto::from(updated);
            
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Estado de solicitud actualizado".to_string()),
            }))
        }
        None => Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        )),
    }
}

pub async fn delete_loan(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    
    let repository = pool.get_repository();
    
    let deleted = repository.delete(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<String> {
            success: true,
            data: None,
            message: Some("Solicitud de préstamo eliminada".to_string()),
        }))
    } else {
        Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        ))
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/loans")
            .route("", web::post().to(create_loan))
            .route("", web::get().to(get_all_loans))
            .route("/{id}", web::get().to(get_loan))
            .route("/{id}/status", web::put().to(update_loan_status))
            .route("/{id}", web::delete().to(delete_loan))
    );
}