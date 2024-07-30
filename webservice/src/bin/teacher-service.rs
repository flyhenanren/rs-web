use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

/**
 * 引入模块，并指明路径
 */

#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path="../state.rs"]
mod state;
#[path = "../models/mod.rs"]
mod models;
#[path ="../db_access/mod.rs"]
mod db_access;
#[path ="../error.rs"]
mod error;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()>{

  dotenv().ok();

  let url = env::var("DATABASE_URL").expect("找不到环境变量中的数据库信息");
  
  // 引入数据库
  let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();
  
  // 初始化共享数据
  let shared_data = web::Data::new(AppState {
    health_check_response: "I'm Ok.".to_string(),
    visit_count: Mutex::new(0),
    db: db_pool
  });

  let app = move || {
    App::new()
    .app_data(shared_data.clone())   // 将数据绑定到内存中
    .configure(general_routers)
    .configure(course_routes)
  };

  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}