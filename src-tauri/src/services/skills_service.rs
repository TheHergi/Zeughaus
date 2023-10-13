use crate::{db::establish_db_connection, models::skills_model::Skill, schema::skills::dsl};
use diesel::prelude::*;
use dsl::skills;

pub fn get_basic_skills() -> Vec<Skill>{
    let connection = &mut establish_db_connection();

    return skills
        .select(Skill::as_select())
        .filter(dsl::is_advanced.eq(0))
        .load(connection)
        .expect("Error loading posts");
}