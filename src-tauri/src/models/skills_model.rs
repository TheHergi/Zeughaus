use diesel::prelude::*;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::skills)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Skill {
    pub skill_id: i32,
    pub title: String,
    pub attribute: String,
    pub is_advanced: Option<i32>,
    pub is_grouped: Option<i32>,
    pub skill_description: Option<String>
}
