use actix_web::{get, web::Json, HttpRequest, Result};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/")]
pub async fn get_status(req: HttpRequest) -> Result<Json<Status>> {
    let ip = req
        .connection_info()
        .peer_addr()
        .unwrap_or("unknown")
        .to_string();

    Ok(Json(Status {
        time: get_epoch_ms(),
        ipv4: ip,
    }))
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Serialize)]
pub struct Status {
    time: u128,
    ipv4: String,
}
