use diesel::prelude::*;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::talents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Talent {
    pub id: i32,
    pub title: String,
    pub maximum: Option<String>,
    pub tests: Option<String>,
    pub talent_description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::talents)]
pub struct NewTalent<'a> {
    pub title: &'a String,
    pub maximum: Option<String>,
    pub tests: Option<String>,
    pub talent_description: Option<String>,
}
