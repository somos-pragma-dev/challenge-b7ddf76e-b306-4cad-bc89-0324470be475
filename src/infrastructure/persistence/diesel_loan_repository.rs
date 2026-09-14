use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{PgConnection, Insertable, Queryable, AsChangeset};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use crate::infrastructure::persistence::schema::loan_requests;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Queryable, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = loan_requests)]
#[diesel(treat_none_as_default = false)]
pub struct LoanRequestEntity {
    pub id: String,
    pub request_id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<LoanRequest> for LoanRequestEntity {
    fn from(req: LoanRequest) -> Self {
        let now = Utc::now();
        LoanRequestEntity {
            id: req.id().to_string(),
            request_id: req.request_id().to_string(),
            applicant_name: req.applicant_name().to_string(),
            amount: req.amount(),
            term_months: req.term_months(),
            status: req.status().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<LoanRequestEntity> for LoanRequest {
    fn from(entity: LoanRequestEntity) -> Self {
        let status = LoanStatus::from(entity.status.as_str());
        LoanRequest::restore(
            entity.id,
            entity.request_id,
            entity.applicant_name,
            entity.amount,
            entity.term_months,
            status,
            entity.created_at,
            entity.updated_at,
        )
    }
}

pub struct DieselLoanRepository {
    pool: DbPool,
}

impl DieselLoanRepository {
    pub fn new(pool: DbPool) -> Self {
        DieselLoanRepository { pool }
    }

    fn get_connection(&self) -> Result<DbConnection, RepositoryError> {
        self.pool
            .get()
            .map_err(|e| RepositoryError::ConnectionError(e.to_string()))
    }
}

#[async_trait]
impl LoanRepository for DieselLoanRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = LoanRequestEntity::from(loan_request);
            
            diesel::insert_into(loan_requests::table)
                .values(&entity)
                .execute(&conn)
                .map_err(|e| {
                    if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                        RepositoryError::Duplicate(format!("Loan request with id {} already exists", entity.request_id))
                    } else {
                        RepositoryError::DatabaseError(e.to_string())
                    }
                })?;
            
            Ok::<_, RepositoryError>(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(LoanRequest::from(result))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::id.eq(&id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        
        let results = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entities = loan_requests::table
                .order(loan_requests::created_at.desc())
                .load::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entities)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(results.into_iter().map(LoanRequest::from).collect())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        let entity = LoanRequestEntity::from(loan_request);
        
        let updated_entity = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::update(loan_requests::table)
                .filter(loan_requests::id.eq(&entity.id))
                .set((
                    loan_requests::applicant_name.eq(&entity.applicant_name),
                    loan_requests::amount.eq(entity.amount),
                    loan_requests::term_months.eq(entity.term_months),
                    loan_requests::status.eq(&entity.status),
                    loan_requests::updated_at.eq(Utc::now()),
                ))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            if rows == 0 {
                return Err(RepositoryError::NotFound(
                    format!("Loan request with id {} not found", entity.id)
                ));
            }
            
            let updated = loan_requests::table
                .filter(loan_requests::id.eq(&entity.id))
                .first::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(updated)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(LoanRequest::from(updated_entity))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let deleted = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::delete(loan_requests::table)
                .filter(loan_requests::id.eq(&id))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            Ok(rows > 0)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(deleted)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let exists = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let count: i64 = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .count()
                .get_result(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(count > 0)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(exists)
    }
}

pub mod schema {
    pub mod loan_requests {
        diesel::table! {
            loan_requests (id) {
                id -> Text,
                request_id -> Text,
                applicant_name -> Text,
                amount -> Float8,
                term_months -> Integer,
                status -> Text,
                created_at -> Timestamp,
                updated_at -> Timestamp,
            }
        }
    }
}