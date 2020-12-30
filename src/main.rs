#[macro_use]
extern crate diesel;
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

mod auth;
mod models;
mod schema;
mod repositories;

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
        // .expect(format!("Cannot find Rustacean with id= {}", id).as_str());
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<Rustacean>)
                          -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::create_rustacean(&c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
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
async fn delete_rustacean(id: i32, conn: DbConn, _auth: BasicAuth) -> JsonValue {
    conn.run(move |c| {
        let result = RustaceanRepository::delete_rustacean(&c, id)
            .expect("Failed to delete rustacean!");
        json!(result)
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
        .launch()
        .await;
}
