use actix_web::{get, HttpResponse};
use chrono::Utc;
use sailfish::TemplateOnce;
use crate::model::flight::Flight;
use crate::model::home_page::Home;

pub const APPLICATION_JSON: &str = "application/json";
#[get("/flights")]
pub async fn list() -> HttpResponse {

    let flight = Flight { code: String::from("0123"), id: String::from("1"), airline: String::from("SK"), created_at: Utc::now() };
    let flights = vec![flight];
    HttpResponse::Ok().body(Home{ flights : flights }.render_once().unwrap())
}