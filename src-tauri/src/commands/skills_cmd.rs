use crate::services::skills_service;
use crate::models::skills_model::Skill;

#[tauri::command]
pub fn get_basic_skills() -> Vec<Skill>
{
    skills_service::get_basic_skills()
}