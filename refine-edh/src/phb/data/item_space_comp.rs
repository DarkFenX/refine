use serde::Deserialize;
use serde_with::{Map, serde_as};

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemSpaceComp {
    #[serde(rename = "systemWideEffects", default)]
    system_wide_effects: Option<PItemSpaceCompSw>,
    #[serde(rename = "systemDbuffEmitter", default)]
    system_dbuff_emitter: Option<PItemSpaceCompSe>,
    #[serde(rename = "appliedProximityEffects", default)]
    applied_proximity_effects: Option<PItemSpaceCompPe>,
    #[serde(rename = "proximityTrap", default)]
    proximity_trap: Option<PItemSpaceCompPt>,
    #[serde(rename = "linkWithShip", default)]
    link_with_ship: Option<PItemSpaceCompSl>,
}
impl KeyMergeOne<rc::ed::EItemBuff> for PItemSpaceComp {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(key),
            system_wide_buffs: self.system_wide_effects.and_then(|v| v.global_debuffs).map(|data| {
                rc::ed::EItemBuffData {
                    buffs: data
                        .dbuffs
                        .into_iter()
                        .map(|(id, value)| rc::ed::EItemBuffEntry {
                            id: rc::ed::EBuffId::from_i32(id),
                            value: rc::ed::EFloat::from_f64(value),
                        })
                        .collect(),
                    item_list_filter: data.eligible_type_list_id.map(rc::ed::EItemListId::from_i32),
                }
            }),
            system_emitter_buffs: self.system_dbuff_emitter.map(|data| rc::ed::EItemBuffData {
                buffs: data
                    .dbuff_collections
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: None,
            }),
            proxy_effect_buffs: self.applied_proximity_effects.map(|data| rc::ed::EItemBuffData {
                buffs: data
                    .effects
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: None,
            }),
            proxy_trigger_buffs: self.proximity_trap.map(|data| rc::ed::EItemBuffData {
                buffs: data
                    .dbuffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: data.trigger_filter_type_list_id.map(rc::ed::EItemListId::from_i32),
            }),
            ship_link_buffs: self.link_with_ship.map(|data| rc::ed::EItemBuffData {
                buffs: data
                    .dbuffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemBuffEntry {
                        id: rc::ed::EBuffId::from_i32(id),
                        value: rc::ed::EFloat::from_f64(value),
                    })
                    .collect(),
                item_list_filter: data.linkable_ship_type_list_id.map(rc::ed::EItemListId::from_i32),
            }),
        }]
    }
}

#[derive(Deserialize)]
struct PItemSpaceCompSw {
    #[serde(rename = "globalDebuffs", default)]
    global_debuffs: Option<PItemSpaceCompSwGlobal>,
}

#[serde_as]
#[derive(Deserialize)]
struct PItemSpaceCompSwGlobal {
    #[serde_as(as = "Map<_, _>")]
    #[serde(default)]
    dbuffs: Vec<(i32, f64)>,
    #[serde(rename = "eligibleTypeListID", default)]
    eligible_type_list_id: Option<i32>,
}

#[serde_as]
#[derive(Deserialize)]
struct PItemSpaceCompSe {
    #[serde_as(as = "Map<_, _>")]
    #[serde(rename = "dbuffCollections", default)]
    dbuff_collections: Vec<(i32, f64)>,
}

#[serde_as]
#[derive(Deserialize)]
struct PItemSpaceCompPe {
    #[serde_as(as = "Map<_, _>")]
    #[serde(default)]
    effects: Vec<(i32, f64)>,
}

#[serde_as]
#[derive(Deserialize)]
struct PItemSpaceCompPt {
    #[serde_as(as = "Map<_, _>")]
    #[serde(default)]
    dbuffs: Vec<(i32, f64)>,
    #[serde(rename = "triggerFilterTypeListID", default)]
    trigger_filter_type_list_id: Option<i32>,
}

#[serde_as]
#[derive(Deserialize)]
struct PItemSpaceCompSl {
    #[serde_as(as = "Map<_, _>")]
    dbuffs: Vec<(i32, f64)>,
    #[serde(rename = "linkableShipTypeListID", default)]
    linkable_ship_type_list_id: Option<i32>,
}
