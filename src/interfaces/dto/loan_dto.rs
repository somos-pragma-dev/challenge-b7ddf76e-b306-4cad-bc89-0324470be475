use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::models::loan_request::LoanRequest;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLoanRequestDto {
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: String,
    
    #[validate(range(min = 100, max = 10000, message = "El monto debe estar entre 100 y 10000"))]
    pub amount: f64,
    
    #[validate(range(min = 6, max = 60, message = "El plazo debe estar entre 6 y 60 meses"))]
    pub term_months: i32,
    
    #[validate(uuid(message = "Debe ser un UUID válido"))]
    pub request_id: String,
}

impl CreateLoanRequestDto {
    pub fn to_entity(self) -> Result<LoanRequest, String> {
        LoanRequest::new(
            self.applicant_name,
            self.amount,
            self.term_months,
            self.request_id,
        ).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLoanStatusDto {
    #[validate(length(min = 1, message = "El estado es requerido"))]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanResponseDto {
    pub id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub request_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<LoanRequest> for LoanResponseDto {
    fn from(req: LoanRequest) -> Self {
        LoanResponseDto {
            id: req.request_id.clone().unwrap_or_default(),
            applicant_name: req.applicant_name,
            amount: req.amount,
            term_months: req.term_months,
            request_id: req.request_id.unwrap_or_default(),
            status: req.status.to_string(),
            created_at: req.created_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: req.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: Option<String>) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message,
        }
    }
    
    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanListResponseDto {
    pub success: bool,
    pub data: Vec<LoanResponseDto>,
    pub total: usize,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanQueryParams {
    #[validate(range(min = 0, message = "El monto mínimo debe ser positivo"))]
    pub min_amount: Option<f64>,
    
    #[validate(range(min = 0, message = "El monto máximo debe ser positivo"))]
    pub max_amount: Option<f64>,
    
    pub status: Option<String>,
    
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: Option<String>,
}

impl Default for LoanQueryParams {
    fn default() -> Self {
        LoanQueryParams {
            min_amount: None,
            max_amount: None,
            status: None,
            applicant_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_loan_request_dto_to_entity_valid() {
        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            amount: 5000.0,
            term_months: 24,
            request_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        };
        
        let result = dto.to_entity();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_create_loan_request_dto_to_entity_invalid_amount() {
        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            amount: 50.0,
            term_months: 24,
            request_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        };
        
        let result = dto.to_entity();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data", Some("Success message".to_string()));
        
        assert!(response.success);
        assert!(response.data.is_some());
        assert!(response.message.is_some());
    }
    
    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<String>::error("Error message".to_string());
        
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.message.is_some());
    }
}