use actix_web::{HttpResponse, web};
use chrono::Utc;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use crate::model::flight::Flight;
use crate::model::home_page::Home;
use log::{error, debug};

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Serialize, Deserialize)]
pub struct FlightParam {
    pub id: String,
    pub airline : Option<String>,
    pub flight_code : Option<String>
}

pub async fn list() -> HttpResponse {
    error!("{}", "Its fleece was white as snow");
    HttpResponse::Ok().body(Home{ flights : get_flights() }.render_once().unwrap())
}

pub async fn delete(data: web::Path<FlightParam>) -> HttpResponse {
    debug!("delete: {}",data.id);
    HttpResponse::Ok().body(Home{ flights : get_flights() }.render_once().unwrap())
}

pub async fn insert(data: String) -> HttpResponse {
    debug!("Insert {}", data);
    HttpResponse::Ok().body(Home{ flights : get_flights() }.render_once().unwrap())
}

fn get_flights() -> Vec<Flight> {
    let flight = Flight { code: String::from("0123"), id: String::from("1"), airline: String::from("SK"), created_at: Utc::now() };
    return vec![flight];
}