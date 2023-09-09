pub mod handlers;

use actix_web::{
    middleware,
    web::{self, resource},
    App, HttpServer,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/app")
                .service(web::scope("/v1").service(web::resource("/ping").to(ping)))
                .service(
                    web::scope("/example")
                        .service(handlers::handler_func::calc_add)
                        .service(
                            resource("/calc/sub/{value1}/{value2}")
                                .route(web::get().to(handlers::handler_func::calc_sub))
                                .route(web::post().to(handlers::handler_func::calc_sub)),
                        ),
                ),
        )
    })
    .bind("127.0.0.1:8081")?
    .workers(2)
    .run()
    .await
}

async fn ping() -> &'static str {
    "pong"
}
