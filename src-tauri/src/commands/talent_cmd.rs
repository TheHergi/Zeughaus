use crate::services::talent_service;
use crate::models::talent_model::{NewTalent, Talent};

#[tauri::command]
pub async fn create_talent(title: String, maximum: Option<String>, tests: Option<String>, talent_description: Option<String>)
{
    let talent_item = NewTalent {title: &title, maximum, tests, talent_description };
    talent_service::store_new_talent(talent_item)
}

#[tauri::command]
pub fn get_talent(talent_id: i32) -> Option<Talent>
{
    talent_service::get_talent(talent_id)
}

#[tauri::command]
pub fn get_talents() -> Vec<Talent>
{
    talent_service::get_talents()
}