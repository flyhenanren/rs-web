use crate::db_access::course::*;
use crate::error::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_course_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|coruses| HttpResponse::Ok().json(coruses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_coruse_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_coruse_detail(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_detail_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::ResponseError;
    use actix_web::{http::StatusCode, web};
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::{env, sync::Mutex};

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("找不到数据库连接信息");
        let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course.".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginnner".into()),
        });

        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_course_success() {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("找不到数据库连接信息");
        let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id = web::Path::from(1);
        let response = get_course_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_one_coruse_success() {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("找不到数据库连接信息");
        let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let param = web::Path::from((1, 1));
        let response = get_course_detail(app_state, param).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[actix_rt::test]

    async fn get_one_coruse_failure() {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("找不到数据库连接信息");
        let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let param = web::Path::from((1, 100));
        let response = get_course_detail(app_state, param).await;
        match response {
            Ok(_) => println!("Something wrong..."),
            Err(error) => assert_eq!(error.status_code(), StatusCode::OK),
        }
    }

    #[actix_rt::test]
    async fn update_coruse_success() {
        let update_coruse = UpdateCourse{ 
            name: Some("ChangeName".into()), 
            description: Some("ChangeDescription".into()), 
            format: Some("ChangeFormat".into()), 
            structure: Some("ChangeStructrue".into()), 
            duration: Some("ChangeDuration".into()), 
            price: None, 
            language: Some("Chinese".into()), 
            level: None 
        };
        
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("找不到数据库连接信息");
        let db_pool = PgPoolOptions::new().connect(&url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let param = web::Path::from((1, 2));
        let response = update_coruse_detail(app_state, web::Json(update_coruse), param).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }
}
