# axtic web tutorial
## 현재 내 구성 상황
- ssh로 원격 개발 중
- ssh의 os는 ubuntu 22 버전
- ubuntu에 docker를 설치 후, docker compose로 환경을 만들어 vscode의 devcontainer로 그 도커 안으로 들어가서 개발
- 즉 외부에서 접근시, 외부 -> ssh -> docker container라는 접근 방식을 가지게 됨.
## 문제 1
### 외부에서 접근 제한
- 서버는 정상적으로 작동을 했으나, 외부에서는 접근을 할 수 없는 문제가 발생.
- 외부 뿐만 아니라 ssh에서 docker container의 8080포트로 접근을 할 수 없었음
### 해결
- /backend/src/main.rs의 파일 중
```rust
HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))

    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
```
- 위의 부분중 bind의 부분이 문제.
- 원래는 127.0.0.1:8080을 사용했지만, 이렇게 하면 내부에서만 접근 가능
- 나는 외부에서도 접근이 가능해야 했기 때문에 0.0.0.0:8080으로 변경하여 해결

## 부하 테스트 하기
```bash
sudo apt install apache2-utils

ab -n 100000 -k -c 30 -q http://localhost:8080/

ab -p ./backend/todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos
```
- -n: 몇 번 반복을 하겠다.
- -k: HTTP KeepAlive 기능 사용. TCP연결을 사용하여 여러번의 HTTP 요청을 보낼 수 있음
- -c: 동시에 요청을 보내는 횟수
- -q: 진행 상황은 출력하지 않음
- -p: post로 어떤 파일을 보내겠다
- -T: Content-Type을 지정하겠다