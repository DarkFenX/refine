use crate::{
    defs::{Count, EEffectId, EItemId, SolFitId, SolItemId},
    sol::{
        info::{SolAutochargeInfo, SolProjInfo},
        uad::item::{SolFighter, SolItemState},
    },
    util::StMap,
};

pub struct SolFighterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
    pub count_override: Option<Count>,
    pub autocharges: StMap<EEffectId, SolAutochargeInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolFighterInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolItemState,
        count_override: Option<Count>,
        autocharges: StMap<EEffectId, SolAutochargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            count_override,
            autocharges,
            projs,
        }
    }
    pub(in crate::sol) fn from_fighter_and_autocharges(
        sol_fighter: &SolFighter,
        autocharges: StMap<EEffectId, SolAutochargeInfo>,
    ) -> Self {
        SolFighterInfo::new(
            sol_fighter.get_id(),
            sol_fighter.get_type_id(),
            sol_fighter.get_fit_id(),
            sol_fighter.get_state(),
            sol_fighter.get_count_override(),
            autocharges,
            sol_fighter
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
