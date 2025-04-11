use misc::check_skill_level;
pub use sole_add_skill::AddSkillError;
pub use sole_get_fit_skills::GetFitSkillsError;
pub use sole_get_skill::GetSkillInfoError;
pub use sole_remove_skill::RemoveSkillError;
pub use sole_set_skill_level::SetSkillLevelError;
pub use sole_set_skill_state::SetSkillStateError;

mod misc;
mod sole_add_skill;
mod sole_get_fit_skills;
mod sole_get_skill;
mod sole_remove_skill;
mod sole_set_skill_level;
mod sole_set_skill_state;
