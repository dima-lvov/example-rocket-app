use super::schema::rustaceans;
use super::schema::rustacean_note;

#[table_name="rustaceans"]
#[derive(serde::Serialize, serde::Deserialize, Insertable, AsChangeset, Queryable)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[table_name="rustacean_note"]
#[derive(serde::Serialize, serde::Deserialize, Insertable, AsChangeset, Queryable)]
pub struct RustaceanNote {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub text: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}
