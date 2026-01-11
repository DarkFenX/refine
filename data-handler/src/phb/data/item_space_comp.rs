use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceComp {
    #[serde(rename = "systemWideEffects", default)]
    pub(in crate::phb) system_wide_effects: Option<PItemSpaceCompSw>,
    #[serde(rename = "systemDbuffEmitter", default)]
    pub(in crate::phb) system_dbuff_emitter: Option<PItemSpaceCompSe>,
    #[serde(rename = "appliedProximityEffects", default)]
    pub(in crate::phb) applied_proximity_effects: Option<PItemSpaceCompPe>,
    #[serde(rename = "proximityTrap", default)]
    pub(in crate::phb) proximity_trap: Option<PItemSpaceCompPt>,
    #[serde(rename = "linkWithShip", default)]
    pub(in crate::phb) link_with_ship: Option<PItemSpaceCompSl>,
}
impl FsdMerge<rc::ed::EItemSpaceComp> for PItemSpaceComp {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSpaceComp> {
        vec![rc::ed::EItemSpaceComp {
            item_id: rc::ed::EItemId::from_i32(id),
            system_wide_buffs: self.system_wide_effects.and_then(|v| v.global_debuffs).map(|data| {
                rc::ed::EItemSpaceCompBuffData {
                    buffs: data
                        .buffs
                        .into_iter()
                        .map(|(id, value)| rc::ed::EItemSpaceCompBuffEntry {
                            id: rc::ed::EBuffId::from_i32(id),
                            value: rc::ed::EFloat::from_f64(value),
                        })
                        .collect(),
                    item_list_filter: data.item_list_filter.map(rc::ed::EItemListId::from_i32),
                }
            }),
            system_emitter_buffs: self.system_dbuff_emitter.map(|data| rc::ed::EItemSpaceCompBuffData {
                buffs: data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: None,
            }),
            proxy_effect_buffs: self
                .applied_proximity_effects
                .map(|data| rc::ed::EItemSpaceCompBuffData {
                    buffs: data
                        .buffs
                        .into_iter()
                        .map(|(id, value)| rc::ed::EItemSpaceCompBuffEntry {
                            id: rc::ed::EBuffId::from_i32(id),
                            value: rc::ed::EFloat::from_f64(value),
                        })
                        .collect(),
                    item_list_filter: None,
                }),
            proxy_trigger_buffs: self.proximity_trap.map(|data| rc::ed::EItemSpaceCompBuffData {
                buffs: data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: data.item_list_filter.map(rc::ed::EItemListId::from_i32),
            }),
            ship_link_buffs: self.link_with_ship.map(|data| rc::ed::EItemSpaceCompBuffData {
                buffs: data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: data.item_list_filter.map(rc::ed::EItemListId::from_i32),
            }),
        }]
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSw {
    #[serde(rename = "globalDebuffs", default)]
    pub(in crate::phb) global_debuffs: Option<PItemSpaceCompSwGlobal>,
}

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSwGlobal {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "dbuffs", default)]
    pub(in crate::phb) buffs: Vec<(i32, f64)>,
    #[serde(rename = "eligibleTypeListID", default)]
    pub(in crate::phb) item_list_filter: Option<i32>,
}

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSe {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "dbuffCollections", default)]
    pub(in crate::phb) buffs: Vec<(i32, f64)>,
}

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompPe {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "effects", default)]
    pub(in crate::phb) buffs: Vec<(i32, f64)>,
}

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompPt {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "dbuffs", default)]
    pub(in crate::phb) buffs: Vec<(i32, f64)>,
    #[serde(rename = "triggerFilterTypeListID", default)]
    pub(in crate::phb) item_list_filter: Option<i32>,
}

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSl {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "dbuffs", default)]
    pub(in crate::phb) buffs: Vec<(i32, f64)>,
    #[serde(rename = "linkableShipTypeListID", default)]
    pub(in crate::phb) item_list_filter: Option<i32>,
}
