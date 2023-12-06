use crate::models::Status;
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{Responder, HttpResponse, web};

// impl: 동적 타입 반환. 여기서 Responder는 여러 타입을 반환할 수 있음
// Responder: HTTP응답을 생성하는데 필요한 기능
// 다양한 타입을 반환할 수 있음. ex) String, static str, json, httpresponse 등
pub async fn status() -> impl Responder { 
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("error connecting to the database");

    // &의 경우 참조를 넘기는 것.
    // 읽기 전용. 이렇게 넘겨 받은 변수의 경우 수정을 하거나 소유권을 가져갈 순 없음
    // 원본 데이터를 가르키는 포인터 이나, 수정이나 소유권을 가질 순 없음
    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}