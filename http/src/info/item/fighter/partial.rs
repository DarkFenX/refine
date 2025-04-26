use std::collections::HashMap;

use rc::{ItemCommon, Lender};

use crate::{
    info::{
        HItemInfoMode,
        item::{autocharge::HAutochargeInfo, proj::HRangedProjInfo},
    },
    shared::{HEffectId, HMinionState},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<(rc::Count, rc::Count)>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) autocharges: HashMap<HEffectId, HAutochargeInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<HRangedProjInfo>,
}
impl HFighterInfoPartial {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            kind: "fighter",
            type_id: core_fighter.get_type_id(),
            fit_id: core_fighter.get_fit().get_fit_id(),
            state: (&core_fighter.get_state()).into(),
            count: core_fighter.get_count().as_ref().map(|v| (v.current, v.max)),
            autocharges: core_fighter
                .iter_autocharges_mut()
                .map_into_iter(|mut autocharge| {
                    (
                        autocharge.get_cont_effect_id().into(),
                        HAutochargeInfo::mk_info(&mut autocharge, item_mode),
                    )
                })
                .collect(),
            projs: core_fighter
                .iter_projs()
                .map(|core_ranged_proj| core_ranged_proj.into())
                .collect(),
        }
    }
}
