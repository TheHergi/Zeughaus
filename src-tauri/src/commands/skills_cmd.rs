use crate::services::skills_service;
use crate::services::skills_spec_service;
use crate::models::skills_model::Skill;
use crate::models::skills_spec_model::SkillSpec;

#[tauri::command]
pub fn get_basic_skills() -> Vec<Skill>
{
    skills_service::get_basic_skills()
}

#[tauri::command]
pub fn get_skill_spec(id: i32) -> Vec<SkillSpec>
{
    skills_spec_service::get_spec_skills(id)
}