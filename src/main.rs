#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::status;
use rocket_contrib::json::JsonValue;

#[get("/rustaceans")]
fn get_rustaceans() -> JsonValue {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "John Doe again" }])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> JsonValue {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean() -> JsonValue {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> JsonValue {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
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
        .launch()
        .await;
}
