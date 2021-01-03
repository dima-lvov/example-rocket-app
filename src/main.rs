#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use auth::BasicAuth;
use models::*;

use crate::repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::fairing::AdHoc;

mod auth;
mod models;
mod schema;
mod repositories;

embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> JsonValue {
    conn.run(|c| {
        let all = RustaceanRepository::load_all(c)
            .expect("Error loading rustaceans from DB");
        json!(all)
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::find_by_id(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::NotFound, json!(e.to_string())))
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<Rustacean>)
                          -> Result<status::Custom<JsonValue>, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::create_rustacean(&c, new_rustacean.into_inner())
            .map(|rustacean| status::Custom(Status::Created, json!(rustacean)))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, conn: DbConn, _auth: BasicAuth)
                          -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::update_rustacean(&c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}


#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, conn: DbConn, _auth: BasicAuth) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        match RustaceanRepository::find_by_id(&c, id) {
            Ok(_) => RustaceanRepository::delete_rustacean(&c, id)
                .map(|_| json!("Done"))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string()))),
            Err(e) => Err(status::Custom(Status::NotFound, json!(e.to_string())))
        }
    }).await
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!("Not found!!!!")
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!("Json payload cannot be converted to target entity!")
}

async fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    DbConn::get_one(&rocket).await
        .expect("failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }).await
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![
            get_rustaceans,
            create_rustacean,
            view_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register(catchers![
            not_found,
            unprocessable_entity
        ])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .launch()
        .await;
}
