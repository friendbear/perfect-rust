use actix_web::{App, HttpServer, middleware, web};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
    }).bind("127.0.0.1:8081")?.workers(2).run().await
}
