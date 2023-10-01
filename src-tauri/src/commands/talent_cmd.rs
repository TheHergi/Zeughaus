use crate::services::talent_service;
use crate::models::talent_model::{NewTalent, Talent};

#[tauri::command]
pub async fn create_talent(title: String, maximum: Option<String>, tests: Option<String>, talent_description: Option<String>)
{
    let talent_item = NewTalent {title: &title, maximum, tests, talent_description };
    talent_service::store_new_talent(talent_item)
}

#[tauri::command]
pub fn get_talent(_id: i32) -> Talent
{
    Talent{id: 5, title: "asd".to_string(), maximum: Some("5".to_string()), tests: Some("aaa".to_string()), talent_description: Some("asdsdadas".to_string())}.into()
}