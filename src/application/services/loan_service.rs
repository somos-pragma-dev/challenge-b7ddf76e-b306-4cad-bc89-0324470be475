use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto};
use crate::interfaces::controllers::errors::{ApiError, RepositoryError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct LoanService<R: LoanRepository> {
    repository: Arc<R>,
}

impl<R: LoanRepository> LoanService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn create_loan(&self, dto: CreateLoanRequestDto) -> Result<LoanResponseDto, ApiError> {
        let exists = self.repository
            .exists_by_request_id(&dto.request_id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists {
            return Err(ApiError::DuplicateResource(
                format!("La solicitud con ID {} ya existe", dto.request_id)
            ));
        }

        let loan_request = LoanRequest::new(
            uuid::Uuid::new_v4().to_string(),
            dto.request_id.clone(),
            dto.applicant_name.clone(),
            dto.amount,
            dto.term_months,
            LoanStatus::Pending,
        );

        if !loan_request.is_valid_amount() {
            return Err(ApiError::ValidationError(
                "El monto debe estar entre 100 y 10000 unidades".to_string()
            ));
        }

        if !loan_request.is_valid_term() {
            return Err(ApiError::ValidationError(
                "El plazo debe estar entre 6 y 60 meses".to_string()
            ));
        }

        let created = self.repository
            .create(loan_request)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(LoanResponseDto::from(created))
    }

    pub async fn get_loan(&self, id: &str) -> Result<LoanResponseDto, ApiError> {
        let loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(req) => Ok(LoanResponseDto::from(req)),
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn get_all_loans(&self) -> Result<Vec<LoanResponseDto>, ApiError> {
        let loans = self.repository
            .find_all()
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(loans.into_iter()
            .map(LoanResponseDto::from)
            .collect())
    }

    pub async fn update_loan_status(&self, id: &str, status: LoanStatus) -> Result<LoanResponseDto, ApiError> {
        let mut loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(ref mut req) => {
                req.update_status(status);
                let updated = self.repository
                    .update(req.clone())
                    .await
                    .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
                Ok(LoanResponseDto::from(updated))
            },
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn delete_loan(&self, id: &str) -> Result<bool, ApiError> {
        let exists = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists.is_none() {
            return Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            ));
        }

        self.repository
            .delete(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))
    }
}