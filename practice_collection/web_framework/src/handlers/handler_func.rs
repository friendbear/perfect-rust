use actix_web::{get, web, HttpResponse, Responder};
use mime;
#[get("/calc/add/{value1}/{value2}")]
pub async fn calc_add(value: web::Path<(String, String)>) -> impl Responder {
    let v1 = value.0.parse::<i32>().unwrap();
    let v2 = value.1.parse::<i32>().unwrap();

    let response = format!("{v1} + {v2} = {}", v1 + v2);

    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN)
        .body(response)
}
pub async fn calc_sub(value: web::Path<(String, String)>) -> impl Responder {
    let v1 = value.0.parse::<i32>().unwrap();
    let v2 = value.1.parse::<i32>().unwrap();

    let response = format!("{v1} - {v2} = {}", v1.wrapping_sub(v2));

    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN)
        .body(response)
}
