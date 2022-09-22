use diesel::prelude::*;
use serde::Serialize;
use crate::schema::installation;


#[derive(Queryable,Serialize,Debug,AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = installation)]
pub struct InstallationItem {
    pub id: i32,
    pub order_number: String,
    pub setup_name: String,
    pub order_status: String,
    pub username: String,
    pub domain: String,
}


#[derive(Insertable)]
#[diesel(table_name = installation)]
pub struct NewInstallation {
    pub order_number: String,
    pub setup_name: String,
    pub order_status: String,
    pub username: String,
    pub domain: String,
}
// pub struct NewInstallation<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }