use actix_web::{get, post, put, delete, web, HttpResponse};
use serde::Deserialize;

use crate::models::{IpAddress, NewIpAddress};

#[derive(Deserialize)]
struct IpAddressForm {
    ip_address: String,
    subnet_mask: String,
    gateway: String,
    is_used: bool,
}

#[get("/ip_addresses")]
async fn ip_addresses(tera: web::Data<Tera>) -> HttpResponse {
    let ip_addresses = IpAddress::all().load::<Vec<_>>(&db::connection()).unwrap();
    let mut context = tera.context();
    context.insert("ip_addresses", &ip_addresses);
    let html = tera.render("ip_addresses.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html
