use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::talents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Talent {
    pub id: String,
    pub title: String,
    pub maximum: String,
    pub tests: String,
    pub talent_desc: String
}

// #[derive(Insertable)]
// #[diesel(table_name = talents)]
// pub struct NewTalent<'a> {
//      pub id: &'a i32,
//     pub description: &'a str,
// }
