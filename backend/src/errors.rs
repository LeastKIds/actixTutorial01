use serde::Serialize;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use tokio_postgres::error::DbError;
use std::fmt;


#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {message: Some(message), cause: _, error_type: _} => message.clone(),
            AppError {message: None, cause: _, error_type: AppErrorType::NotFoundError} => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string()
        }
    }

    pub fn db_error(error: impl ToString) -> AppError {
        AppError { message: None, cause: Some(error.to_string()), error_type: AppErrorType::DbError}
    }
}

// impl fmt::Display for AppError는 AppError를 출력했을 때,
// 보여주는 방식을 정의하는 함수. java의 toString함수와 비슷
impl fmt::Display for AppError {
    // ex) AppError { message: Some("An error occurred"), cause: Some("Unknown"), error_type: DbError }
    // 이러한 형태로 터미널에 출력
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String
}


// AppError를 ResponseError의 규칙에 맞게 사용하겠다.
// 다른 언어의 상속 개념과 비슷. 즉 ResponseError에는
// 이미 status_code와 error_response의 메소드가 존재해
// 여기서 우리는 재정의 해서 사용하는것.
// ResponseError는 오류가 발생했을 때, 우리가 재정의한 이 값들을 가지고
// 알맞게 에러로 표현해 줌
// 여기서 &self는 AppError를 뜻함
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(AppErrorResponse {error: self.message()})
    }
}