use crate::services::skills_service;
use crate::services::skills_spec_service;
use crate::models::skills_model::Skill;
use crate::models::skills_spec_model::SkillSpec;

#[tauri::command]
pub fn get_skill(id: i32) -> Option<Skill>
{
    skills_service::get_skill(id)
}

#[tauri::command]
pub fn get_skills(advanced: bool) -> Vec<Skill>
{
    skills_service::get_skills(advanced)
}

#[tauri::command]
pub fn get_skill_spec(id: i32) -> Vec<SkillSpec>
{
    skills_spec_service::get_spec_skills(id)
}

#[tauri::command]
pub fn get_skill_specs() -> Vec<SkillSpec>
{
    skills_spec_service::get_skill_specs()
}