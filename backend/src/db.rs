use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    // await를 써야하는지 아닌지는 타입을 체크해 보거나 직접 경험을 해 보는 수 밖에 없다.
    // statment: sql query를 준비하는데 사용하는 변수.
    // query를 최적화 시켜 주고 문제는 없는지 체크한다.
    let statement = client.prepare("select * from todo_list order by id desc").await.unwrap();


    let todos = client.query(&statement, &[])
                .await
                .expect("Error getting todo lists")
                .iter()
                // 여기서 row는 각 각의 행을 뜻함. map으로 배열을 하나씩 분리 했을 때 나오는 하나의 객체
                // 이렇게 분리된 값을 from_row_ref(row)라는 함수로 TodoList라는 객체로 변환
                .map(|row| TodoList::from_row_ref(row).unwrap())
                // 그렇게 각각의 오브젝트를 다시 배열로 전환
                // Vec: 동적 배열
                .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_itmes(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {

    let statement = client.prepare("select * from todo_item where list_id = $1 order by id").await.unwrap();

    let itmes = client.query(&statement, &[&list_id])
                                        .await
                                        .expect("Error getting todo lists")
                                        .iter()
                                        .map(|row| TodoItem::from_row_ref(row).unwrap())
                                        .collect::<Vec<TodoItem>>();

    Ok(itmes)
}