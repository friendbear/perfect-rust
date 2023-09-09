use serde::{Deserialize, Serialize};
use actix_web::{get, web, HttpResponse, Responder, error};
use mime;
use tera::Tera;


#[derive(Serialize, Deserialize)]
pub struct CalcForm{
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub opt: Option<String>,
}
impl CalcForm {
    pub fn calc(&self) -> anyhow::Result<i32> {
        let func = |str_value: &String| ->i32 {
            if str_value.eq("") {
                return 0;
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

pub async fn calc_get(tera: web::Data<Tera>) -> impl Responder {
    log::info!("{:?}", tera);
    let resp_body = tera.render("pages/result.html", &tera::Context::new())
        .map_err(|err| error::ErrorInternalServerError(err.to_string()));
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body.unwrap())
}

pub async fn calc_post(form: web::Form<CalcForm>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let calc_form = form.into_inner();
    match calc_form.calc() {
        Ok(result) => context.insert("result", &result),
        Err(err) => context.insert("result", &err.to_string()),
    }
    let resp_body = tera.render("pages/result.html", &context)
        .map_err(|err| error::ErrorInternalServerError(err.to_string())).unwrap();
    log::info!("{:?}", resp_body);
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body)
}