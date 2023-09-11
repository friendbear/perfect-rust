
use mime;
use actix_web::{get, web, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode};
use serde::{Serialize, Deserialize};

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
#[derive(Serialize, Deserialize)]
pub struct AddCalc{
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub answer: Option<String>,
}
impl AddCalc {

    pub fn calc(&mut self) {
        let func = |v: &String| ->i32 {
            if v.eq("") {
                return 0
            } else {
                v.parse::<i32>().unwrap()
            }
        };
        let value1 = self.value1.as_ref().map_or_else(|| 0, func);
        let value2 = self.value2.as_ref().map_or_else(|| 0, func);
        self.value1 = Some(value1.to_string());
        self.value2 = Some(value2.to_string());
        self.answer = Some((value1 + value2).to_string());
    }
}
impl ToString for AddCalc {
    fn to_string(&self) -> String {
        format!("{} + {} = {}", self.value1.as_ref().unwrap(), self.value2.as_ref().unwrap(),
                self.answer.as_ref().unwrap())
    }
}

pub async fn calc_add_from_query(query: web::Query<AddCalc>) -> impl Responder {
    let mut query_value = query.into_inner();
    query_value.calc();
    log::info!("{:?}", query_value.to_string());
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(query_value.to_string())
}
pub async fn calc_add_from_form(form: web::Form<AddCalc>) -> impl Responder {
    let mut form_value = form.into_inner();
    form_value.calc();
    log::info!("{:?}", form_value.to_string());
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(form_value.to_string())
}
pub async fn calc_add_from_json(json: web::Json<AddCalc>) -> impl Responder {
    let mut json_value = json.into_inner();
    json_value.calc();
    log::info!("{:?}", json_value.to_string());
    HttpResponse::Ok().content_type(mime::APPLICATION_JSON)
        .status(StatusCode::OK)
        .insert_header(("X-Application", "acix-web"))
        .json(json_value.to_string())
}