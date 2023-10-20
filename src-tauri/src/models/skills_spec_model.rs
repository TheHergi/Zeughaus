use diesel::prelude::*;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::skills_spec)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SkillSpec {
    pub skill_spec_id: i32,
    pub title: String,
    pub skill_id: Option<i32>
}