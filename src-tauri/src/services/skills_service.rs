use crate::{
    db::establish_db_connection,
    models::skills_model::Skill,
    schema::skills,
    schema::skills::dsl
};

use diesel::prelude::*;

pub fn get_basic_skills() -> Vec<Skill>{
    let connection = &mut establish_db_connection();

    return dsl::skills
        .select(Skill::as_select())
        .filter(skills::is_advanced.eq(0))
        .load(connection)
        .expect("Error get_basic_skills");
}
