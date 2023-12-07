use std::io;

use crate::models::{Status, CreateTodoList, ResultResponse};
use crate::db;
use crate::errors::{AppError, AppErrorType};
use deadpool_postgres::{Pool, Client};
use actix_web::{Responder, HttpResponse, web};

// impl: 동적 타입 반환. 여기서 Responder는 여러 타입을 반환할 수 있음
// Responder: HTTP응답을 생성하는데 필요한 기능
// 다양한 타입을 반환할 수 있음. ex) String, static str, json, httpresponse 등
pub async fn status() -> impl Responder { 
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get()
        .await
        // 만약 에러가 발생했을 시, |err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError}를 발생시킴
        // ?의 경우 에러가 발생했을 시, 에러를 표출하고 그 즉시 행위를 멈춤
        // .map_err(|err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError})?;
        .map_err(AppError::db_error)?;

    // &의 경우 참조를 넘기는 것.
    // 읽기 전용. 이렇게 넘겨 받은 변수의 경우 수정을 하거나 소유권을 가져갈 순 없음
    // 원본 데이터를 가르키는 포인터 이나, 수정이나 소유권을 가질 순 없음
    let result = db::get_todos(&client).await;

    // result는 현재 Result<Vec<TodoList>, AppError>의 타입을 가지고 있다.
    // 이것을 Result<json 값을 가지고있는 Vec<TodoList>>로 바꾸는 형 변환 과정이다
    // rust에서는 map을 사용하면 some이나 ok 상태의 값을 안전하게 다룰 수 있다. 
    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_itmes(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {

    let client: Client = db_pool.get()
    .await
    // .map_err(|err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError})?;
    .map_err(AppError::db_error)?;
    

    let result = db::get_itmes(&client, path.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
}

// CreateTodoList에 #[derive(Serialize, Deserialize)]가 설정되어 있고, web::json으로 가져온다면
// 자동으로 clone 기능 같은것이 따라오는것 같다.
pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {

    let client: Client = db_pool.get()
        .await
        .map_err(AppError::db_error)?;

    let result = db::create_todo(&client, json.title.clone()).await;

    // match result {
    //     Ok(todo) => HttpResponse::Ok().json(todo),
    //     Err(_) => HttpResponse::InternalServerError().into()
    // }
    result.map(|todo| HttpResponse::Ok().json(todo))
}

pub async fn check_itme(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {

    let client: Client = db_pool.get()
        .await
        .map_err(AppError::db_error)?;

    let result = db::check_item(&client, path.0, path.1).await;

    // match result {
    //     Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
    //     Err(ref e) if e.kind() == io::ErrorKind::Other => HttpResponse::Ok().json(ResultResponse{success: false}),
    //     Err(_) => HttpResponse::InternalServerError().into()
    // }
    result.map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
}