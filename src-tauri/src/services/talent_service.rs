use crate::{db::establish_db_connection, models::talent_model::NewTalent, schema::talents::dsl};
use diesel::prelude::*;

pub fn store_new_talent(talent: NewTalent) {
    print!("store new talent");
    let connection = &mut establish_db_connection();

    diesel::insert_into(dsl::talents)
        .values(&talent)
        .execute(connection)
        .expect("Error saving new talent");
}