use rc::{ItemCommon, Lender};
use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::info::item::item_booster::side_effect::HSideEffectInfo;

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<i32>,
    enabled: bool,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    side_effects: Vec<(rc::EffectId, HSideEffectInfo)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterInfoPartial {
    pub(super) fn from_core(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
            kind: "booster",
            type_id: core_booster.get_type_id().into_i32(),
            fit_id: core_booster.get_fit().get_fit_id(),
            slot: core_booster.get_slot().map(|v| v.into_i32()),
            enabled: core_booster.get_state(),
            side_effects: core_booster
                .iter_side_effects_mut()
                .map_into_iter(|core_side_effect| {
                    (
                        core_side_effect.get_effect_id(),
                        HSideEffectInfo::from_core(core_side_effect),
                    )
                })
                .collect(),
        }
    }
}
