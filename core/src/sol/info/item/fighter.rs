use std::collections::HashMap;

use crate::sol::{
    AdjustableCount, EffectId, FitId, ItemId, ItemTypeId,
    info::{AutochargeInfo, ProjInfo},
    uad::{
        Uad,
        item::{Fighter, MinionState},
    },
};

pub struct FighterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    pub count: Option<AdjustableCount>,
    pub autocharges: HashMap<EffectId, AutochargeInfo>,
    pub projs: Vec<ProjInfo>,
}
impl FighterInfo {
    pub(in crate::sol) fn from_fighter(uad: &Uad, fighter: &Fighter) -> Self {
        let mut autocharges = HashMap::new();
        for (a_effect_id, &autocharge_key) in fighter.get_autocharges().iter() {
            let autocharge = uad.items.get(autocharge_key).get_autocharge().unwrap();
            autocharges.insert(a_effect_id.into(), AutochargeInfo::from_autocharge(uad, autocharge));
        }
        Self {
            id: fighter.get_item_id(),
            type_id: fighter.get_a_item_id(),
            fit_id: uad.fits.id_by_key(fighter.get_fit_key()),
            state: fighter.get_fighter_state(),
            count: fighter.get_count(),
            autocharges,
            projs: fighter
                .get_projs()
                .iter()
                .map(|(&projectee_item_key, &range)| ProjInfo {
                    item_id: uad.items.id_by_key(projectee_item_key),
                    range,
                })
                .collect(),
        }
    }
}
