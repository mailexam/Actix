mod mail;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use mail::MailConfig;
use serde::Deserialize;

#[derive(Deserialize)]
struct SendRequest {
    to: String,
    subject: Option<String>,
    body: Option<String>,
}

#[post("/mail/test")]
async fn send_mail(
    config: web::Data<MailConfig>,
    payload: web::Json<SendRequest>,
) -> impl Responder {
    mail::send_test(
        &config,
        &payload.to,
        payload.subject.as_deref().unwrap_or("Actix + Mailexam"),
        payload.body.as_deref().unwrap_or("Mailexam test from Actix"),
    )
    .await
    .expect("smtp send");

    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = web::Data::new(MailConfig::from_env());

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT");

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(send_mail)
    })
    .bind((bind_addr.as_str(), port))?
    .run()
    .await
}
