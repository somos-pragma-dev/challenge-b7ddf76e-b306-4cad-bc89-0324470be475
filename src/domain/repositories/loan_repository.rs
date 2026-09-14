use crate::domain::models::loan_request::LoanRequest;
use crate::interfaces::controllers::errors::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait LoanRepository: Send + Sync {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError>;
    
    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn delete(&self, id: &str) -> Result<bool, RepositoryError>;
    
    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct LoanFilter {
    pub status: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub applicant_name_contains: Option<String>,
}

impl Default for LoanFilter {
    fn default() -> Self {
        LoanFilter {
            status: None,
            min_amount: None,
            max_amount: None,
            applicant_name_contains: None,
        }
    }
}

impl LoanFilter {
    pub fn with_status(status: String) -> Self {
        LoanFilter {
            status: Some(status),
            ..Default::default()
        }
    }
    
    pub fn with_amount_range(min: f64, max: f64) -> Self {
        LoanFilter {
            min_amount: Some(min),
            max_amount: Some(max),
            ..Default::default()
        }
    }
    
    pub fn with_applicant_name(name: String) -> Self {
        LoanFilter {
            applicant_name_contains: Some(name),
            ..Default::default()
        }
    }
}