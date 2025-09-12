mod model;

use std::str::FromStr;

use reef_tracing::SentryTracing;
use model::APIService;


#[tokio::main]
async fn main() {
    // 加载配置
    let settings = settings::Settings::new().unwrap();

    //初始化日志
    let _tracing = SentryTracing::init(&settings, "api");
    

    let service = std::env::args().nth(1).unwrap_or_default();
    let service = APIService::from_str(service.as_str()).ok().unwrap_or(APIService::Api);

    println!("api start service: {}", service.as_ref());

}
