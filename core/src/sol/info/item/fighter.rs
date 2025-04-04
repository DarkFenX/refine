use crate::{
    sol::{
        AdjustableCount, EffectId, FitId, ItemId, ItemTypeId,
        info::{AutochargeInfo, ProjInfo},
        uad::item::{Fighter, MinionState},
    },
    util::RMap,
};

pub struct FighterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    pub count: Option<AdjustableCount>,
    pub autocharges: RMap<EffectId, AutochargeInfo>,
    pub projs: Vec<ProjInfo>,
}
impl FighterInfo {
    pub(in crate::sol) fn from_fighter_and_autocharges(
        sol_fighter: &Fighter,
        autocharges: RMap<EffectId, AutochargeInfo>,
    ) -> Self {
        Self {
            id: sol_fighter.get_item_id(),
            type_id: sol_fighter.get_a_item_id(),
            fit_id: sol_fighter.get_fit_id(),
            state: sol_fighter.get_fighter_state(),
            count: sol_fighter.get_count(),
            autocharges,
            projs: sol_fighter
                .get_projs()
                .iter()
                .map(|(&item_id, &range)| ProjInfo { item_id, range })
                .collect(),
        }
    }
}
