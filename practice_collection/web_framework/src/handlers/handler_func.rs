
use actix_web::{get, Responder, web, HttpResponse};
use mime;
#[get("/calc/sum/{value1}/{value2}")]
pub async fn calc_sum(value: web::Path<(String, String)>) -> impl Responder {
    let v1 = value.0.parse::<i32>().unwrap();
    let v2 = value.1.parse::<i32>().unwrap();

    let response = format!("{v1} + {v2} = {}", v1 + v2);

    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(response)
}