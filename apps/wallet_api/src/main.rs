mod chain;
mod model;
mod response;
mod status;

use actix_web::{middleware::Logger, web, App, HttpServer};
use model::APIService;
use reef_tracing::SentryTracing;
use settings::Settings;
use settings_chain::ChainProviders;
use std::str::FromStr;
use tokio::sync::Mutex;

async fn create_app(settings: Settings) -> std::io::Result<()> {
    let bind_address = format!("{}:{}", settings.server.address, settings.server.port);
    let chain_providers =
        web::Data::new(Mutex::new(ChainProviders::from_settings(&settings).await));

    HttpServer::new(move || {
        App::new()
            .app_data(chain_providers.clone())
            .wrap(Logger::default())
            .service(status::get_status)
            .service(chain::balance::get_balances)
            .service(chain::token::get_token)
    })
    .bind(&bind_address)?
    .run()
    .await
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

    //启动Actix-web API
    match service {
        APIService::WebsocketPrices => {
            // TODO 时使用 API 服务
            create_app(settings).await.expect("Failed to launch server");
        }
        APIService::Api => {
            create_app(settings).await.expect("Failed to launch server");
        }
    }
}
