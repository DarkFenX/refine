use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolStructure,
};

pub struct SolStructureInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolStructureInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolStructure> for SolStructureInfo {
    fn from(sol_ship: &SolStructure) -> Self {
        SolStructureInfo::new(
            sol_ship.id,
            sol_ship.fit_id,
            sol_ship.a_item_id,
            sol_ship.get_bool_state(),
        )
    }
}
