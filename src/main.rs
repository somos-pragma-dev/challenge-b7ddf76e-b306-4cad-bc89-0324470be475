use actix_web::{web, App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod domain;
mod application;
mod infrastructure;
mod interfaces;
mod config;

use config::database::establish_connection;
use interfaces::controllers::loan_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let bind_address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    println!("Iniciando servidor en {}", bind_address);
    println!("Conectando a base de datos: {}", database_url);
    
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .idle_timeout(std::time::Duration::from_secs(600))
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Failed to create pool");
    
    let pool_clone = pool.clone();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_clone.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(loan_controller::configure)
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn health_check() -> actix_web::Result<impl actix_web::Responder> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-management-api",
        "version": "1.0.0"
    })))
}