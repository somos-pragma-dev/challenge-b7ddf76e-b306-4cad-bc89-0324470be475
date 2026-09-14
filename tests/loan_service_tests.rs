use loan_management_api::domain::models::loan_request::{LoanRequest, LoanStatus};
use loan_management_api::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use loan_management_api::application::services::loan_service::LoanService;
use loan_management_api::interfaces::dto::loan_dto::CreateLoanRequestDto;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockRepository {
    loans: Arc<Mutex<Vec<LoanRequest>>>,
    should_fail: Arc<Mutex<bool>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            loans: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    fn with_initial_data(initial: Vec<LoanRequest>) -> Self {
        Self {
            loans: Arc::new(Mutex::new(initial)),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl LoanRepository for MockRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        loans.push(loan_request.clone());
        Ok(loan_request)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.id.as_str() == id).cloned())
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.request_id.as_str() == request_id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.clone())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        if let Some(idx) = loans.iter().position(|l| l.id == loan_request.id) {
            loans[idx] = loan_request.clone();
            return Ok(loan_request);
        }
        Err(RepositoryError::NotFound(format!("Loan with id {} not found", loan_request.id)))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        let len_before = loans.len();
        loans.retain(|l| l.id.as_str() != id);
        Ok(loans.len() < len_before)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().any(|l| l.request_id.as_str() == request_id))
    }
}

#[tokio::test]
async fn test_service_creates_loan_successfully() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Juan Pérez".to_string(),
        amount: 5000.0,
        term_months: 24,
        request_id: "REQ-001".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.applicant_name, "Juan Pérez");
    assert_eq!(loan.amount, 5000.0);
    assert_eq!(loan.term_months, 24);
}

#[tokio::test]
async fn test_service_rejects_amount_below_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Ana López".to_string(),
        amount: 50.0,
        term_months: 12,
        request_id: "REQ-002".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("monto") || err.to_string().contains("amount"));
}

#[tokio::test]
async fn test_service_rejects_amount_above_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Carlos García".to_string(),
        amount: 50000.0,
        term_months: 12,
        request_id: "REQ-003".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_term_below_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "María Rodríguez".to_string(),
        amount: 1000.0,
        term_months: 3,
        request_id: "REQ-004".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_term_above_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Pedro Sánchez".to_string(),
        amount: 1000.0,
        term_months: 120,
        request_id: "REQ-005".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_duplicate_request_id() {
    let existing_loan = LoanRequest::new(
        "1".to_string(),
        "EXISTING-REQ".to_string(),
        "Cliente Existente".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing_loan]);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Nuevo Cliente".to_string(),
        amount: 2000.0,
        term_months: 6,
        request_id: "EXISTING-REQ".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("duplic"));
}

#[tokio::test]
async fn test_service_allows_duplicate_request_id_after_deletion() {
    let existing_loan = LoanRequest::new(
        "1".to_string(),
        "REQ-TO-DELETE".to_string(),
        "Cliente a Eliminar".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing_loan]);
    let service = LoanService::new(Arc::new(mock_repo.clone()));
    
    let delete_result = service.delete_loan("1".to_string()).await;
    assert!(delete_result.is_ok());
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Nuevo Cliente".to_string(),
        amount: 2000.0,
        term_months: 6,
        request_id: "REQ-TO-DELETE".to_string(),
    };
    
    let create_result = service.create_loan(dto).await;
    assert!(create_result.is_ok());
}

#[tokio::test]
async fn test_service_get_all_loans() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
        LoanRequest::new("3".to_string(), "REQ-3".to_string(), "Cliente 3".to_string(), 3000.0, 36, LoanStatus::Rejected),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_all_loans().await;
    assert!(result.is_ok());
    let loans = result.unwrap();
    assert_eq!(loans.len(), 3);
}

#[tokio::test]
async fn test_service_get_loan_by_id() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_loan_by_id("1".to_string()).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.id, "1");
}

#[tokio::test]
async fn test_service_get_loan_by_id_not_found() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_loan_by_id("nonexistent".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_update_loan_status() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.update_loan_status("1".to_string(), LoanStatus::Approved).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.status, LoanStatus::Approved);
}

#[tokio::test]
async fn test_service_delete_loan() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.delete_loan("1".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_amount_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 100.0,
        term_months: 12,
        request_id: "REQ-BOUNDARY-1".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_amount_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 10000.0,
        term_months: 12,
        request_id: "REQ-BOUNDARY-2".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_term_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 5000.0,
        term_months: 6,
        request_id: "REQ-BOUNDARY-3".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_term_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 5000.0,
        term_months: 60,
        request_id: "REQ-BOUNDARY-4".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}