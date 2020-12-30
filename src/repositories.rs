use diesel::prelude::*;

use crate::models::Rustacean;
use crate::schema::rustaceans;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn load_all(c: &SqliteConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(100).load::<Rustacean>(c)
    }

    pub fn find_by_id(c: &SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn delete_rustacean(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    pub fn create_rustacean(c: &SqliteConnection, new_rustacean: Rustacean) -> QueryResult<Rustacean> {
        match diesel::insert_into(rustaceans::table)
            // .values(new_rustacean)
            .values((
                rustaceans::name.eq(new_rustacean.name.to_owned()),
                rustaceans::email.eq(new_rustacean.email.to_owned()),
            ))
            .execute(c) {
            Ok(_) => rustaceans::table.order(rustaceans::id.desc()).limit(1).get_result::<Rustacean>(c),
            Err(e) => Err(e)
        }
    }

    pub fn update_rustacean(c: &SqliteConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((rustaceans::email.eq(rustacean.email.to_owned()),
                  rustaceans::name.eq(rustacean.name.to_owned())))
            .execute(c)?;
        Self::find_by_id(c, id)
    }
}

