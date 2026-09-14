pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod config;

pub use domain::models::loan_request::{LoanRequest, LoanStatus, ValidationError};
pub use domain::repositories::loan_repository::{LoanRepository, LoanFilter};
pub use application::services::loan_service::LoanService;
pub use infrastructure::persistence::diesel_loan_repository::{DieselLoanRepository, DbPool};
pub use interfaces::controllers::errors::{ApiError, ApiResponse};
pub use interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, UpdateLoanStatusDto, LoanListResponseDto};