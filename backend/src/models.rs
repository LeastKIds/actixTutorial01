use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

// attribute
// 보통 그 아래에 있는 함수를 attribute의 설정에 맞춰 구현을 자동 생성해 줌
// 혹은 #[inline], #[allow], #[warn]은 경우는 컴파일러에게 특정한 지시를 내릴 수 있음
// #[test]의 경우는 테스트 함수라는 의미를 가지고 있음
// #[cfg]은 조건에 따라 코드를 포함하거나 제외하는 기능을 가지고 있음
// 직렬화(Serialize): 객체를 네트워크 전송등을 할 수 있는 값으로 바꾸어 줌
// ex) 자바의 객체를 json 문자열로 변환
#[derive(Serialize)]
pub struct Status {
    pub status: String
}

// Serialize, Deserialize가 모두 선언되어 있는 이유는
// 데이터베이스이기 때문에 json -> object, object -> json
// 으로 바뀌는 경우가 모두 있기 때문이다.
// PostgresMapper를 사용하기 위해서는 
// #[pg_mapper(table="todo_list")] 와 같이 테이블 설정을 해 줘야만 한다.
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool
}