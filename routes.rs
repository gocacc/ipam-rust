#[post("/ip_addresses")]
async fn create_ip_address(
    form: web::Form<IpAddressForm>,
    db: web::Data<PgConnection>,
) -> HttpResponse {
    let new_ip_address = NewIpAddress {
        ip_address: form.ip_address.clone(),
        subnet_mask: form.subnet_mask.clone(),
        gateway: form.gateway.clone(),
        is_used: form.is_used,
    };

    new_ip_address.insert(&db::connection()).unwrap();

    HttpResponse::SeeOther()
        .header("Location", "/ip_addresses")
        .finish()
}

#[put("/ip_addresses/{id}")]
async fn update_ip_address(
    id: web::Path<i32>,
    form: web::Form<IpAddressForm>,
    db: web::Data<PgConnection>,
) -> HttpResponse {
    let ip_address = IpAddress::find(id.into_inner()).load::<IpAddress>(&db::connection()).unwrap();

    let updated_ip_address = IpAddress {
        id: ip_address.id,
        ip_address: form.ip_address.clone(),
        subnet_mask: form.subnet_mask.clone(),
        gateway: form.gateway.clone(),
        is_used: form.is_used,
    };

    updated_ip_address.update(&db::connection()).unwrap();

    HttpResponse::SeeOther()
        .header("Location", "/ip_addresses")
        .finish()
}

#[delete("/ip_addresses/{id}")]
async fn delete_ip_address(id: web::Path<i32>, db: web::Data<PgConnection>) -> HttpResponse {
    IpAddress::find(id.into_inner()).delete(&db::connection()).unwrap();

    HttpResponse::SeeOther()
        .header("Location", "/ip_addresses")
        .finish()
}
