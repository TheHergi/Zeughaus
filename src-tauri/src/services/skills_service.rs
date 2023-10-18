use crate::{
    db::establish_db_connection,
    models::skills_model::Skill,
    schema::skills,
    schema::skills::dsl
};

use diesel::prelude::*;

pub fn get_skills(advanced: bool) -> Vec<Skill> {
    let connection = &mut establish_db_connection();

    return dsl::skills
        .select(Skill::as_select())
        .filter(skills::is_advanced.eq(advanced as i32))
        .load(connection)
        .expect("Error get_basic_skills");
}

pub fn get_skill(id: i32) -> Option<Skill> {
    let connection = &mut establish_db_connection();

    return dsl::skills
        .select(Skill::as_select())
        .filter(skills::skill_id.eq(id))
        .first(connection)
        .optional()
        .unwrap()
}