use actix_web::{test, web, App};
use loan_management_api::interfaces::controllers::loan_controller::configure_loan_routes;
use loan_management_api::interfaces::controllers::errors::ApiError;
use loan_management_api::domain::models::loan_request::{LoanRequest, LoanStatus};
use loan_management_api::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use loan_management_api::application::services::loan_service::LoanService;
use loan_management_api::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, LoanListResponseDto, ApiResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

struct MockRepository {
    loans: Arc<Mutex<Vec<LoanRequest>>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            loans: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn with_initial_data(initial: Vec<LoanRequest>) -> Self {
        Self {
            loans: Arc::new(Mutex::new(initial)),
        }
    }
}

#[async_trait::async_trait]
impl LoanRepository for MockRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let mut loans = self.loans.lock().await;
        loans.push(loan_request.clone());
        Ok(loan_request)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loan_request.iter().find(|l| l.id.as_str() == id).cloned())
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.request_id.as_str() == request_id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.clone())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let mut loans = self.loans.lock().await;
        if let Some(idx) = loans.iter().position(|l| l.id == loan_request.id) {
            loans[idx] = loan_request.clone();
            return Ok(loan_request);
        }
        Err(RepositoryError::NotFound(format!("Loan with id {} not found", loan_request.id)))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let mut loans = self.loans.lock().await;
        let len_before = loans.len();
        loans.retain(|l| l.id.as_str() != id);
        Ok(loans.len() < len_before)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.iter().any(|l| l.request_id.as_str() == request_id))
    }
}

#[actix_web::test]
async fn test_create_loan_endpoint_success() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 24,
            "request_id": "REQ-TEST-001"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status().is_created());
}

#[actix_web::test]
async fn test_create_loan_endpoint_invalid_amount() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 50.0,
            "term_months": 24,
            "request_id": "REQ-TEST-002"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_endpoint_invalid_term() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 3,
            "request_id": "REQ-TEST-003"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_endpoint_duplicate() {
    let existing = LoanRequest::new(
        "1".to_string(),
        "REQ-DUP".to_string(),
        "Cliente Existente".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing]);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Nuevo Cliente",
            "amount": 2000.0,
            "term_months": 6,
            "request_id": "REQ-DUP"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_get_all_loans_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_loan_by_id_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans/1")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_loan_by_id_not_found() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans/nonexistent")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_not_found());
}

#[actix_web::test]
async fn test_update_loan_status_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::put()
        .uri("/api/loans/1/status")
        .set_json(&json!({
            "status": "approved"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_delete_loan_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::delete()
        .uri("/api/loans/1")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status().is_no_content());
}

#[actix_web::test]
async fn test_create_loan_missing_applicant_name() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "amount": 5000.0,
            "term_months": 24,
            "request_id": "REQ-TEST-004"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_missing_request_id() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 24
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_negative_amount() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": -100.0,
            "term_months": 24,
            "request_id": "REQ-TEST-005"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_zero_term() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 0,
            "request_id": "REQ-TEST-006"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_get_all_loans_empty() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}