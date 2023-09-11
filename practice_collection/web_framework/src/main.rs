pub mod handlers;
use actix_web::{
    middleware,
    web::{self, resource, ServiceConfig},
    App, HttpServer,
};
use handlers::tera_handler;
use tera::Tera;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .service(
                web::scope("/app")
                    .service(web::scope("/v1").service(web::resource("/ping").to(ping)))
                    .configure(set_configure),
            )
    })
    .bind("127.0.0.1:8081")?
    //.workers(2) // default Number of physical cores
    .run()
    .await
}
fn set_configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/examples")
            .service(handlers::handler_func::calc_add)
            .service(
                resource("/calc/sub/{value1}/{value2}")
                    .route(web::get().to(handlers::handler_func::calc_sub))
                    .route(web::post().to(handlers::handler_func::calc_sub)),
            )
            .service(
                resource("/calc_form")
                    .route(web::get().to(tera_handler::calc_get))
                    .route(web::post().to(tera_handler::calc_post)),
            )
            .service(
                resource("/calc")
                    .route(web::get().to(handlers::handler_func::calc_add_from_query))
                    .route(web::post().to(handlers::handler_func::calc_add_from_form)),
            )
            .service(
                resource("/calc-json")
                    .route(web::post().to(handlers::handler_func::calc_add_from_json)),
            ),
    );
}

async fn ping() -> &'static str {
    "pong"
}
