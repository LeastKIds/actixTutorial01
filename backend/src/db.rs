use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    // await를 써야하는지 아닌지는 타입을 체크해 보거나 직접 경험을 해 보는 수 밖에 없다.
    // statment: sql query를 준비하는데 사용하는 변수.
    // query를 최적화 시켜 주고 문제는 없는지 체크한다.
    let statement = client.prepare("select * from todo_list order by id desc limit 10").await.unwrap();


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

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let statement = client.prepare("insert into todo_list (title) values ($1) returning id, title").await.unwrap();

    client.query(&statement, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        // 위의 값을 보면 하나의 값만을 반환한다. 하지만 아래에서는 map과 collection으로 억지로 배열로 만든다.
        // 그 이유는 client.query는 무조건 vec 형태로 반환하기 때문
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo list"))
}

pub async fn check_item(cleint: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {

    // set chcked = true 라는 소리는 checked 항목을 true로 바꾸겠다는 소리
    let statement = cleint.prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false").await.unwrap();

    // 결과물은 업데이트 된 todo의 수
    // 1개가 없데이터 되었다면 결과 값은 1
    let result = cleint.execute(&statement, &[&list_id, &item_id])
                                                        .await
                                                        .expect("Error checking todo item");

    match result {
        // ref는 참조를 발생. 즉 result의 값을 updated에 참조 시키는 것
        // * 는 역참조를 뜻함. 즉 *updated는 updated의 실제 값을 가져오는 것
        // 언뜻 보기에는 참조를 한 뒤, 역참조를 하는것 처럼 보여, 의미 없는 작업처럼 보일 수 있으나
        // result의 값을 그대로 사용할 시, result의 소유권이 check_item이 아니라 match쪽으로 넘어간다.
        // 그렇게 되면 result는 match 안에서는 사용할 수 있으나, 더이상 check_item에서는 사용할 수 없게 된다.
        // 그렇기에 result의 값만 가져와 새로운 변수에 저장하기 위해서는
        // 우선 참조를 한 뒤, 역 참조를 해 그 값만 가져올 수 밖에 없다.
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list"))
    }
}