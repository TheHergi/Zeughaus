use crate::{db::establish_db_connection, models::talent_model::Talent, schema::talents::dsl};
use diesel::prelude::*;

pub fn store_new_talent(talent: &Talent) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(dsl::talents)
        .values(talent)
        .execute(connection)
        .expect("Error saving new talent");
}