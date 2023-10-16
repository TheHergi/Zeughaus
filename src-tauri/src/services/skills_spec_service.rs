use crate::{
    db::establish_db_connection,
    models::skills_spec_model::SkillSpec,
    schema::skills_spec,
    schema::skills_spec::dsl
};

use diesel::prelude::*;

pub fn get_spec_skills(id: i32) -> Vec<SkillSpec> {
    let connection = &mut establish_db_connection();

    return dsl::skills_spec
        .select(SkillSpec::as_select())
        .filter(skills_spec::skill_id.eq(id))
        .load(connection)
        .expect("Error get_spec_skills");
}

pub fn get_skill_specs() -> Vec<SkillSpec> {
    let connection = &mut establish_db_connection();
    return dsl::skills_spec
        .select(SkillSpec::as_select())
        .load(connection)
        .expect("Error get_spec_skills"); 
}