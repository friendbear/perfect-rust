use actix_web::{
    middleware,
    web::{self, resource, ServiceConfig},
    cookie::time::Duration,
    App, HttpServer, HttpResponse, Responder,
};
use actix_session::config::BrowserSession;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .wrap(
            SessionMiddleware::builder(
                CookieSessionStore::default(), key.clone()
            )
            .session_lifecycle(
                BrowserSession::default().state_ttl(Duration::minutes(5))
            )
            .cookie_name("rsessionid".to_string())
            .build()
        )
        .service(
            web::scope("/v2")
            .configure(set_configure)
        )
        .route("/", web::get().to(|| HttpResponse::Ok()))
    }).bind_openssl("127.0.0.1:8082", create_ssl_accepter_builder())?
    .run()
    .await
}

fn set_configure(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/health")
        .route(web::get().to(health))
    );
}
async fn health() -> impl Responder {

    HttpResponse::Ok().content_type(mime::APPLICATION_JSON).json(r#"{"health": "Ok"}"#)
}
// openssl crate
fn create_ssl_accepter_builder() -> SslAcceptorBuilder {
    // OpenSSL構造を管理し、暗号スイート、セッションオプションなどを構成する
    let mut builder: SslAcceptorBuilder =
        SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server()).unwrap();
    builder.set_private_key_file("local-key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("local.pem").unwrap();
    builder
}