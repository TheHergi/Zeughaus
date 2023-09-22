use crate::services::talent_service;
use crate::models::talent_model::Talent;

use uuid::Uuid;

#[tauri::command]
pub async fn create_talent(name: String)
{
    let talent_item = Talent {
        id: Uuid::new_v4().to_string(),
        title: name,
        maximum: String::from("3"),
        tests: String::from("myNewMax"),
        talent_desc: String::from("myVeryFirstDesc"),
    };
    talent_service::store_new_talent(&talent_item)
}