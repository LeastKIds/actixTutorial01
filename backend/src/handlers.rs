use std::io;

use crate::config::AppState;
use crate::models::{Status, CreateTodoList, ResultResponse};
use crate::db;
use crate::errors::{AppError, AppErrorType};
use deadpool_postgres::{Pool, Client};
use actix_web::{Responder, HttpResponse, web};
use slog::{o, crit, Logger, error};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get()
        .await
        .map_err(|err| {
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error creating client");

            AppError::db_error(err)
        })
}

// 일반 클로저나 함수로 사용하지 않는 이유는 dyn을 사용하기 위해
// dyn는 동적 디스패치
// 컴파일을 할 때 체크하는것이 아닌, 컴파일을 하고 난 뒤, 이 함수가 사용될 때 불려짐
// 정적 디스패치(평범한 함수나 클로저)를 사용하지 않는 이유는
// 이 클로저를 좀 더 유연하게 모든 환경에서 사용하기 위함
// 클로저의 경우 밖에 있는 변수등을 사용할수 있는데
// 정적 디스패치에서는 이를 지정해 줘야만 사용할 수 있다.
// 현재 이 클로저는 모든 라우터에서 사용되기 때문에 미리 지정해 줄 수 가 없다.
// 그래서 동적 디스패치를 사용해야만 하고, 그러기 위해서는 
// 이런식으로 구성이 되어야 한다.

// 클로저의 경우 같은 스코프에 있는 변수를 따로 받지 않아도 사용할 수 있다.
// 그래서 AppError의 경우 파라미터로 받지 않아도 사용할 수 있다.
// 하지만 여기서는 log를 파라미터로 받았는데
// 이는 더욱 안전하게 log를 사용하기 위함이다.
// 클로저가 같은 스코프에 있는 변수를 캡처(사용)하면
// 클로저가 없어질 때까지 다른곳에서 해당 변수를 사용할 수 없다
// 다만 이 경우는 move를 사용하고 있기 때문에 캡처를 하나 파라미터로 받나
// log의 수명은 이 클로저가 사라질 때 함께 없어지게 된다
pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
            error!(sublog, "{}", err.message());
            err
    })
}

// impl: 동적 타입 반환. 여기서 Responder는 여러 타입을 반환할 수 있음
// Responder: HTTP응답을 생성하는데 필요한 기능
// 다양한 타입을 반환할 수 있음. ex) String, static str, json, httpresponse 등
pub async fn status() -> impl Responder { 
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    
    // log 위치 설정등
    // 여기서 handler는 마음대로 정해도 되는 양식
    // 내가 포현하고 싶은 값을 적당한 변수명으로 출력 가능
    let log = state.log.new(o!("handler" => "get_todos"));
    
    // let client: Client = state.pool.get()
    //     .await
    //     // 만약 에러가 발생했을 시, |err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError}를 발생시킴
    //     // ?의 경우 에러가 발생했을 시, 에러를 표출하고 그 즉시 행위를 멈춤
    //     // .map_err(|err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError})?;
    //     .map_err(|err| {
    //         // 언제 어디서든 추가로 로그에 기록 가능
    //         let sublog = log.new(o!("cause" => err.to_string()));
    //         // 크리티컬 에러 표시
    //         crit!(sublog, "Error creating client");
    //         AppError::db_error(err)
    //     })?;

    // 모든 곳에서 client를 사용하기 때문에 따로 함수로 빼서
    // 동일한 코드의 중복성 해소
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;



    // &의 경우 참조를 넘기는 것.
    // 읽기 전용. 이렇게 넘겨 받은 변수의 경우 수정을 하거나 소유권을 가져갈 순 없음
    // 원본 데이터를 가르키는 포인터 이나, 수정이나 소유권을 가질 순 없음
    let result = db::get_todos(&client).await;

    // result는 현재 Result<Vec<TodoList>, AppError>의 타입을 가지고 있다.
    // 이것을 Result<json 값을 가지고있는 Vec<TodoList>>로 바꾸는 형 변환 과정이다
    // rust에서는 map을 사용하면 some이나 ok 상태의 값을 안전하게 다룰 수 있다. 
    result
        .map(|todos| HttpResponse::Ok().json(todos))
        // .map_err(|err| {
        //     let sublog = log.new(o!("cause" => err.cause.clone()));
        //     error!(sublog, "{}", err.message());
        //     err
        // })
        // 모든곳에서 사용하기 위해 동적 디스패치로 만듬
        // 여기서 error 값이 자동으로 입력 됨.
        .map_err(log_error(log))
}

pub async fn get_itmes(state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {

    // let client: Client = state.pool.get()
    // .await
    // // .map_err(|err| AppError{message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError})?;
    // .map_err(AppError::db_error)?;

    let log = state.log.new(o!("handler" => "get_itmes"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    

    let result = db::get_itmes(&client, path.0).await;

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}

// CreateTodoList에 #[derive(Serialize, Deserialize)]가 설정되어 있고, web::json으로 가져온다면
// 자동으로 clone 기능 같은것이 따라오는것 같다.
pub async fn create_todo(state: web::Data<AppState>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {

    // let client: Client = state.pool.get()
    //     .await
    //     .map_err(AppError::db_error)?;

    let log = state.log.new(o!("handler" => "create_todo"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::create_todo(&client, json.title.clone()).await;

    // match result {
    //     Ok(todo) => HttpResponse::Ok().json(todo),
    //     Err(_) => HttpResponse::InternalServerError().into()
    // }
    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(log))
}

pub async fn check_itme(state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {

    // let client: Client = state.pool.get()
    //     .await
    //     .map_err(AppError::db_error)?;

    let log = state.log.new(o!("handler" => "check_item"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::check_item(&client, path.0, path.1).await;

    // match result {
    //     Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
    //     Err(ref e) if e.kind() == io::ErrorKind::Other => HttpResponse::Ok().json(ResultResponse{success: false}),
    //     Err(_) => HttpResponse::InternalServerError().into()
    // }
    result
        .map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
        .map_err(log_error(log))
}