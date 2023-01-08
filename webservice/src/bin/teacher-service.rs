use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::sync::Mutex;
use crate::error::MyError;

use dotenv::dotenv;
use std::env;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../error.rs"]
mod error;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Player provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };
    
    println!("Service is running");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
