mod config;
mod models; // 해당 파일을 사용
mod handlers;
mod db;
// mod의 경우 최상위에서 한 번 사용하면,
// 하위 파일에서는 굳이 mod로 불러올 필요 없이
// use crate로 가져와서 쓰면 된다.

use actix_web::{HttpServer, App, web::{self, Data}};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::{Runtime};
use crate::handlers::*; // 그렇게 정의된 모듈, 타입, 함수 등을 현재 범위로 가져와 사용가능하게 함


// actix_rt는 비동기 프로그래밍을 위한 라이브러리
// 함수 내에서 async등의 코드를 사용할 수 있게 해 줌 
#[actix_rt::main]
async fn main() -> io::Result<()>{ 

    // .env파일을 시스템 환경변수에 등록.
    // 이렇게 해야 config라이브러리에서 시스템 환경변수를 불러올 때
    // .env파일에 있는 내용도 등록이 됨
    dotenv().ok();
    // result type은 optional처럼 ok값과 error 값을 가지고 있음.
    // unwrap을 할 때, ok이면 그대로 실행, error면 error 발생하고 프로그램 정지
    let config = crate::config::ConfigSetting::from_env().unwrap();

    // postgres 데이터베이스 설정 파일로 부터 해당 데이터베이스 컨트롤러를 가져오기
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);

    // ::는 모듈 접근. 다른 언어서는 보통 .으로 표현. 예를 들어 std::io의 경우,
    // 다른 언어에서는 std.io로 표현하는 경우가 많음
    
    // ||는 클로저. 함수의 파라미터와는 개념이 약간 다름
    // 파라미터의 경우 밖에 있는 값을 복사해 와서 새로운 변수로 선언해, 함수 안에서 사용하는 것.
    // 그렇기에 함수 안에서 값이 변경 되어도 밖에 있는 변수에는 영향을 주지 않음
    // 반면 클로저는 해당 변수의 참조값을 가져와서 사용.
    // 그렇기에 안 에서 값이 변경이 되었을 때, 밖에 있는 변수에도 영향을 줌.

    // 다만 현재 move || 이 방식은 원본 변수의 소유권을 클로저에게 넘기는 작업을 하게 됨
    // 즉 원본 변수는 소유권이 클로저에게 이전이 되기 때문에, 더이상 사용 불가능. 다만 메모리에서 바로 해제되는것이 아닌, 해당 스코프가 끝날때까지는 메모리에 존재
    // 클로저의 경우는 화살표 함수나 람다식으로 생각하면 됨.
    HttpServer::new(move || {
        App::new()
            // 어플리케이션 레벨에서의 데이터를 설정하는 곳.
            // 데이터베이스 연결 풀을 초기화 시켜, 모든 요청에서 데이터베이스를 사용할 수 있게 설정
            // 이렇게 설정된 데이터베이스 연결 풀은 
            // 각각의 요청에 독립적으로 접근할 수 있게 해줌
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))
            // {_:/?} 는 맨 마지막에 / 뒤에 오는 값들은 무시를 하겠다
            // get_todos의 경우 db_pool: web::Data<Pool>의 파라미터가 필요하지만
            // actix의 경우 app_data안에 있는 값에서 찾아서 자동으로 넣어줌
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_itmes))

    })
    // 만약 bind에 성공하면 그대로 넘어가고 아니면 error 발생
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
