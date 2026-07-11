use super::sol_id::SolarSystemId;

pub(crate) struct SolarSystemInner {
    // TODO: consider if it is needed here, or can be lifted above (after API is refined)
    pub(super) id: SolarSystemId,
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<Box<rc::SolarSystem>>,
}
impl SolarSystemInner {
    pub(super) fn new(core_sol: Box<rc::SolarSystem>) -> Self {
        Self {
            id: SolarSystemId(uuid::Uuid::new_v4()),
            accessed: chrono::Utc::now(),
            core_sol: Some(core_sol),
        }
    }
}
