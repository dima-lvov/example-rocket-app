#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use models::*;
use schema::*;
use auth::BasicAuth;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> JsonValue {
    conn.run(|c| {
        let all = rustaceans::table.limit(100).load::<Rustacean>(c).expect("Error loading rustaceans from DB");
        json!(all)
    }).await
}
#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> JsonValue {
    conn.run(move |c| {
        let rustacean = rustaceans::table.find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean from DB");
        json!(rustacean)
    }).await
}
#[post("/rustaceans", format = "json", data="<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> JsonValue {
    conn.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error adding rustaceans to DB");
        json!(result)
    }).await
}
#[put("/rustaceans/<id>", format = "json", data="<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> JsonValue {
    conn.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating rustaceans to DB");
        json!(result)
    }).await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> status::NoContent {
    conn.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean from DB");
        status::NoContent
    }).await
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!("Not found!")
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
            not_found
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
