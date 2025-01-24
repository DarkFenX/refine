use std::collections::HashMap;

use crate::{
    info::{item::autocharge::HAutochargeInfo, HItemInfoMode},
    shared::HState,
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) state: HState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count_override: Option<rc::Count>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) autocharges: HashMap<rc::EEffectId, HAutochargeInfo>,
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<(rc::SolItemId, Option<rc::AttrVal>)>,
}
impl HFighterInfoPartial {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fighter_info: &rc::SolFighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_fighter_info.id,
            kind: "fighter",
            type_id: core_fighter_info.type_id,
            fit_id: core_fighter_info.fit_id,
            state: (&core_fighter_info.state).into(),
            count_override: core_fighter_info.count_override,
            autocharges: core_fighter_info
                .autocharges
                .iter()
                .map(|(k, v)| (*k, HAutochargeInfo::mk_info(core_sol, v, item_mode)))
                .collect(),
            projs: core_fighter_info.projs.iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
