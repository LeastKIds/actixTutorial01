use serde::Deserialize;
use config::{ConfigError, Config, Environment};

// 역직렬화(Deserialize): 직렬화와는 반대로 데이터를 객체로 변환해 줌
// ex) json 형식을 java의 object형식으로 바꾸어 줌
#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}
#[derive(Deserialize)]
pub struct ConfigSetting {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config
}

// 여기서 impl은 위의 구조체 configsetting이 가지고 있는 기능을 나타냄
impl ConfigSetting {
    // 그렇기 때문에 여기서 self는 위의 구조체 pub struct configsetting을 나타냄
    pub fn from_env() -> Result<Self, ConfigError> {
        // config의 빌더 를 불러오는 부분
        let builder = Config::builder()
        // config에 환경설정 부분을 설정하는 부분.
        // .env파일을 읽는 것이 아닌, 시스템 환경변수를 읽는 것.
            .add_source(Environment::default().separator(".")); // separator의 경우는 현재 .env에는 SERVER.HOSt로 되어있으니 .을 구분으로 나눈다는 뜻

        let cfg = builder.build()?;

        // 위의 값들을 이용해 configsetting의 객체를 생성하려 시도.
        // 에러가 발생하면 configerror 발생
        // rust에서는 마지막 값이 return 값
        // ;도 사용하지 않음
        // 함수 중간에 반환을 하고 싶으면 return을 사용해야 함.
        cfg.try_deserialize::<ConfigSetting>()
    }
}