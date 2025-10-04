// src/main.rs
#![feature(proc_macro_hygiene,decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod routes;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket::serde::{json::Json, Serialize};
use rocket::http::Status;
use rocket::State;
use std::env;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Supplier {
    id: i32,
    name: String,
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewSupplier {
    name: String,
    address: String,
}

#[post("/supplier", format = "json", data = "<new_supplier>")]
fn create_supplier(conn: DbConn, new_supplier: Json<NewSupplier>) -> Json<Supplier> {
    use schema::suppliers;

    let new_supplier: NewSupplier = new_supplier.into_inner();
    let supplier = suppliers::create_supplier(&conn, new_supplier.name, new_supplier.address).expect("Error creating supplier");
    Json(supplier)
}

#[get("/supplier/<id>")]
fn get_supplier(id: i32, conn: DbConn) -> Result<Json<Supplier>, Status> {
    use schema::suppliers::dsl::*;

    let supplier = suppliers.find(id).first::<Supplier>(&conn).optional();
    match supplier {
        Ok(Some(supplier)) => Ok(Json(supplier)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/supplier/<id>")]
fn delete_supplier(id: i32, conn: DbConn) -> Status {
    use schema::suppliers::dsl::*;

    let result = diesel::delete(suppliers.find(id)).execute(&conn);
    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api", routes![create_supplier, get_supplier, delete_supplier])
        .manage(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
}

// src/schema.rs
table! {
    suppliers (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
    }
}

// src/models.rs
use super::schema::suppliers;

#[derive(Insertable)]
#[table_name="suppliers"]]
pub struct NewSupplier<'a> {
    pub name: &'a str,
    pub address: &'a str,
}

impl<'a> NewSupplier<'a> {
    pub fn create_supplier(conn: &PgConnection, name: &'a str, address: &'a str) -> QueryResult<Supplier> {
        use crate::schema::suppliers;

        let new_supplier = NewSupplier {
            name,
            address,
        };

        diesel::insert_into(suppliers::table)
            .values(&new_supplier)
            .get_result(conn)
    }
}

// src/routes.rs
use rocket::get;
use rocket::post;
use rocket::delete;
use super::Supplier;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the supplier management system!"
}
