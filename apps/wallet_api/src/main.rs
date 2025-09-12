mod model;
mod status;

use std::str::FromStr;

use model::APIService;
use reef_tracing::SentryTracing;
use rocket::{fairing::AdHoc, routes, Build, Rocket};
use settings::Settings;

async fn rocket_api(settings: Settings) -> Rocket<Build> {
    let config = rocket::Config {
        port: settings.server.port,
        address: settings.server.address.parse().expect("Failed to parse server address"),
        cli_colors: settings.server.rocket_log_color,
        ..rocket::Config::default()
    };

    rocket::custom(&config)
        .attach(AdHoc::on_ignite(
            "Tokio Runtime Configuration",
            |rocket| async {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime");
                rocket.manage(runtime)
            },
        ))
        .mount("/", routes![status::get_status])
}

#[tokio::main]
async fn main() {
    // 加载配置
    let settings = settings::Settings::new().expect("Failed to load settings");

    //初始化日志 全局
    let _tracing = SentryTracing::init(&settings, "api");

    let service = std::env::args().nth(1).unwrap_or_default();
    let service = APIService::from_str(service.as_str())
        .ok()
        .unwrap_or(APIService::Api);

    println!("api start service: {}", service.as_ref());

    //启动Rocket API

    let rocket_api = match service {
        APIService::WebsocketPrices => {
            // TODO 时使用 API 服务
            rocket_api(settings).await
        }
        APIService::Api => rocket_api(settings).await,
    };
    rocket_api.launch().await.expect("Failed to launch Rocket");
}
