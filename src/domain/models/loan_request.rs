use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LoanStatus {
    Pending,
    Approved,
    Rejected,
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanStatus::Pending => write!(f, "pending"),
            LoanStatus::Approved => write!(f, "approved"),
            LoanStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl From<String> for LoanStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "approved" => LoanStatus::Approved,
            "rejected" => LoanStatus::Rejected,
            _ => LoanStatus::Pending,
        }
    }
}

impl From<&str> for LoanStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "approved" => LoanStatus::Approved,
            "rejected" => LoanStatus::Rejected,
            _ => LoanStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanRequest {
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: String,
    
    #[validate(range(min = 100, max = 10000, message = "El monto debe estar entre 100 y 10000"))]
    pub amount: f64,
    
    #[validate(range(min = 6, max = 60, message = "El plazo debe estar entre 6 y 60 meses"))]
    pub term_months: i32,
    
    #[validate(uuid(message = "El identificador debe ser un UUID válido"))]
    pub request_id: String,
    
    pub id: String,
    
    pub status: LoanStatus,
    
    pub created_at: Option<DateTime<Utc>>,
    
    pub updated_at: Option<DateTime<Utc>>,
}

impl LoanRequest {
    pub fn new(
        id: String,
        request_id: String,
        applicant_name: String,
        amount: f64,
        term_months: i32,
        status: LoanStatus,
    ) -> Self {
        let now = Utc::now();
        LoanRequest {
            id,
            request_id,
            applicant_name,
            amount,
            term_months,
            status,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
    
    pub fn restore(
        id: String,
        request_id: String,
        applicant_name: String,
        amount: f64,
        term_months: i32,
        status: LoanStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        LoanRequest {
            id,
            request_id,
            applicant_name,
            amount,
            term_months,
            status,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
    
    pub fn update_status(&mut self, status: LoanStatus) {
        self.status = status;
        self.updated_at = Some(Utc::now());
    }
    
    pub fn is_valid_amount(&self) -> bool {
        self.amount >= 100.0 && self.amount <= 10000.0
    }
    
    pub fn is_valid_term(&self) -> bool {
        self.term_months >= 6 && self.term_months <= 60
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    
    pub fn applicant_name(&self) -> &str {
        &self.applicant_name
    }
    
    pub fn amount(&self) -> f64 {
        self.amount
    }
    
    pub fn term_months(&self) -> i32 {
        self.term_months
    }
    
    pub fn status(&self) -> &LoanStatus {
        &self.status
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    AmountOutOfRange,
    TermOutOfRange,
    InvalidName,
    MissingRequestId,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::AmountOutOfRange => {
                write!(f, "El monto debe estar entre 100 y 10000 unidades")
            }
            ValidationError::TermOutOfRange => {
                write!(f, "El plazo debe estar entre 6 y 60 meses")
            }
            ValidationError::InvalidName => {
                write!(f, "El nombre del solicitante es inválido")
            }
            ValidationError::MissingRequestId => {
                write!(f, "El identificador de solicitud es requerido")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loan_request_creation_with_valid_data() {
        let request = LoanRequest::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "REQ-001".to_string(),
            "Juan Pérez".to_string(),
            5000.0,
            24,
            LoanStatus::Pending,
        );
        
        assert_eq!(request.applicant_name, "Juan Pérez");
        assert_eq!(request.amount, 5000.0);
        assert_eq!(request.term_months, 24);
    }
    
    #[test]
    fn test_loan_request_status_update() {
        let mut request = LoanRequest::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "REQ-001".to_string(),
            "Juan Pérez".to_string(),
            5000.0,
            24,
            LoanStatus::Pending,
        );
        
        request.update_status(LoanStatus::Approved);
        
        assert_eq!(request.status, LoanStatus::Approved);
        assert!(request.updated_at.is_some());
    }
}