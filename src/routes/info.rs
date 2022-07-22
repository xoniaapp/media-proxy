use actix_web::web;
use actix_web::Responder;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    media_proxy: &'static str,
}

pub async fn get() -> impl Responder {
    web::Json(Info {
        media_proxy: env!("CARGO_PKG_VERSION"),
    })
}
