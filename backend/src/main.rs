mod models; // 해당 파일을 사용

use crate::models::Status;  // 그렇게 정의된 모듈, 타입, 함수 등을 현재 범위로 가져와 사용가능하게 함
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use std::io;

// impl: 동적 타입 반환. 여기서 Responder는 여러 타입을 반환할 수 있음
// Responder: HTTP응답을 생성하는데 필요한 기능
// 다양한 타입을 반환할 수 있음. ex) String, static str, json, httpresponse 등
async fn status() -> impl Responder { 
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

// actix_rt는 비동기 프로그래밍을 위한 라이브러리
// 함수 내에서 async등의 코드를 사용할 수 있게 해 줌 
#[actix_rt::main]
async fn main() -> io::Result<()>{

    println!("Starting server at http://127.0.0.1:8080/");

    // ::는 모듈 접근. 다른 언어서는 보통 .으로 표현. 예를 들어 std::io의 경우,
    // 다른 언어에서는 std.io로 표현하는 경우가 많음
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))

    })
    // 만약 bind에 성공하면 그대로 넘어가고 아니면 error 발생
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
