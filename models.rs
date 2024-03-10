use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name = "ip_addresses"]
pub struct IpAddress {
    id: i32,
    ip_address: String,
    subnet_mask: String,
    gateway: String,
    is_used: bool,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "ip_addresses"]
pub struct NewIpAddress {
    ip_address: String,
    subnet_mask: String,
    gateway: String,
    is_used: bool,
}
