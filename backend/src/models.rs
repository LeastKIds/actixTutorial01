use serde_derive::Serialize;

// attribute
// 보통 그 아래에 있는 함수를 attribute의 설정에 맞춰 구현을 자동 생성해 줌
// 혹은 #[inline], #[allow], #[warn]은 경우는 컴파일러에게 특정한 지시를 내릴 수 있음
// #[test]의 경우는 테스트 함수라는 의미를 가지고 있음
// #[cfg]은 조건에 따라 코드를 포함하거나 제외하는 기능을 가지고 있음
#[derive(Serialize)]
pub struct Status {
    pub status: String
}