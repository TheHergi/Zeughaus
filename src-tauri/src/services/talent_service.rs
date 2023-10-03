use crate::{db::establish_db_connection, models::talent_model::{NewTalent, Talent}, schema::talents::dsl};
use diesel::prelude::*;
use dsl::talents;

pub fn store_new_talent(talent: NewTalent) {
    print!("store new talent");
    let connection = &mut establish_db_connection();

    diesel::insert_into(dsl::talents)
        .values(&talent)
        .execute(connection)
        .expect("Error saving new talent");
}

pub fn get_talents() -> Vec<Talent>{
    let connection = &mut establish_db_connection();

    return talents
        .select(Talent::as_select())
        .load(connection)
        .expect("Error loading posts");
}