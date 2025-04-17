use std::collections::HashMap;

use rc::{ItemCommon, Lender};

use crate::{
    info::{HItemInfoMode, item::autocharge::HAutochargeInfo},
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
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<(rc::ItemId, Option<rc::AttrVal>)>,
}
impl HFighterInfoPartial {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        let mut autocharges = HashMap::new();
        let mut autocharge_iter = core_fighter.iter_autocharges_mut();
        while let Some(mut autocharge) = autocharge_iter.next() {
            autocharges.insert(
                autocharge.get_cont_effect_id().into(),
                HAutochargeInfo::mk_info(&mut autocharge, item_mode),
            );
        }
        Self {
            id: core_fighter.get_item_id(),
            kind: "fighter",
            type_id: core_fighter.get_type_id(),
            fit_id: core_fighter.get_fit().get_fit_id(),
            state: (&core_fighter.get_state()).into(),
            count: core_fighter.get_count().as_ref().map(|v| (v.current, v.max)),
            autocharges,
            projs: core_fighter.get_projs().iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
