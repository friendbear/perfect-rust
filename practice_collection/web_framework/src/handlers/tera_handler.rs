use actix_web::{error, web, HttpResponse, Responder};
use mime;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct CalcForm {
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub opt: Option<String>,
}
impl CalcForm {
    #[allow(dead_code)]
    pub fn calc(&self) -> anyhow::Result<i32> {
        let func = |str_value: &String| -> i32 {
            if str_value.eq("") {
                0
            } else {
                str_value.parse::<i32>().unwrap()
            }
        };
        let v1 = func(self.value1.as_ref().unwrap());
        let v2 = func(self.value2.as_ref().unwrap());
        let opt = func(self.opt.as_ref().unwrap_or(&String::from("-1")));
        let result = match opt {
            1 => v1 + v2,
            2 => v1 - v2,
            3 => v1 * v2,
            4 => v1 / v2,
            5 => v1 % v2,
            _ => return Err(anyhow::Error::msg("Parameter Error.")),
        };
        Ok(result)
    }
}

#[allow(dead_code)]
pub async fn calc_get(tera: web::Data<Tera>) -> impl Responder {
    log::info!("{:?}", tera);
    let resp_body = tera
        .render("pages/result.html", &tera::Context::new())
        .map_err(|err| error::ErrorInternalServerError(err.to_string()));
    HttpResponse::Ok()
        .content_type(mime::TEXT_HTML)
        .body(resp_body.unwrap())
}

#[allow(dead_code)]
pub async fn calc_post(form: web::Form<CalcForm>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let calc_form = form.into_inner();
    match calc_form.calc() {
        Ok(result) => context.insert("result", &result),
        Err(err) => context.insert("result", &err.to_string()),
    }
    let resp_body = tera
        .render("pages/result.html", &context)
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))
        .unwrap();
    log::info!("{:?}", resp_body);
    HttpResponse::Ok()
        .content_type(mime::TEXT_HTML)
        .body(resp_body)
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::web::resource;
    use actix_web::{test, web, App, Error};

    async fn test_service(
    ) -> impl actix_web::dev::Service<actix_http::Request, Response = ServiceResponse, Error = Error>
    {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
        let test_service = test::init_service(
            App::new()
                //.wrap(middleware::Logger::default())
                .app_data(web::Data::new(tera.clone()))
                .service(
                    web::scope("/samples").service(
                        resource("/calc_form")
                            .route(web::get().to(super::calc_get))
                            .route(web::post().to(super::calc_post)),
                    ),
                ),
        )
        .await;
        test_service
    }
    #[actix_web::test]
    async fn calc_get() {
        let test_service = test_service().await;
        let enter_request = test::TestRequest::get()
            .uri("/samples/calc_form")
            .to_request();

        let response = test::call_service(&test_service, enter_request).await;
        println!("{response:?}");
        assert_eq!(StatusCode::OK, response.status())
    }
    #[actix_web::test]
    async fn calc_post() {
        let test_service = test_service().await;
        let post_data = CalcForm {
            value1: Some("150".to_owned()),
            value2: Some("50".to_owned()),
            opt: Some("2".to_owned()),
        };
        let enter_request = test::TestRequest::post()
            .set_form(&post_data)
            .uri("/samples/calc_form")
            .to_request();
        let response = test::call_service(&test_service, enter_request).await;
        println!("{:?}", response.response().body());
        assert_eq!(StatusCode::OK, response.status())
    }
}
