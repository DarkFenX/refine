use std::collections::{BTreeMap, btree_map::Entry};

use serde::Deserialize;

use crate::sde::data::ExtractOne;

pub(in crate::sde) fn merge_item_buffs(
    e_conts: [rc::ed::EDataCont<rc::ed::EItemBuff>; 5],
) -> rc::ed::EDataCont<rc::ed::EItemBuff> {
    let mut item_buffs = BTreeMap::new();
    let mut warnings = Vec::new();
    for e_cont in e_conts {
        merge_e_cont(&mut item_buffs, e_cont.data);
        warnings.extend(e_cont.warnings);
    }
    rc::ed::EDataCont {
        data: item_buffs.into_values().collect(),
        warnings,
    }
}

fn merge_e_cont(merged: &mut BTreeMap<i32, rc::ed::EItemBuff>, data: Vec<rc::ed::EItemBuff>) {
    for item_buff in data.into_iter() {
        match merged.entry(item_buff.item_id.into_i32()) {
            Entry::Vacant(entry) => {
                entry.insert(item_buff);
            }
            Entry::Occupied(mut entry) => {
                let target = entry.get_mut();
                if target.system_wide_buffs.is_none() {
                    target.system_wide_buffs = item_buff.system_wide_buffs;
                }
                if target.system_emitter_buffs.is_none() {
                    target.system_emitter_buffs = item_buff.system_emitter_buffs;
                }
                if target.proxy_effect_buffs.is_none() {
                    target.proxy_effect_buffs = item_buff.proxy_effect_buffs;
                }
                if target.proxy_trigger_buffs.is_none() {
                    target.proxy_trigger_buffs = item_buff.proxy_trigger_buffs;
                }
                if target.ship_link_buffs.is_none() {
                    target.ship_link_buffs = item_buff.ship_link_buffs;
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// System-wide effects
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(in crate::sde) struct SItemBuffSw {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(default)]
    dbuffs: Vec<SItemBuffEntry>,
    #[serde(rename = "eligibleTypeListID")]
    eligible_type_list_id: Option<i32>,
}
impl ExtractOne<rc::ed::EItemBuff> for SItemBuffSw {
    fn extract(self) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(self.item_id),
            system_wide_buffs: Some(into_item_buff_data(self.dbuffs, self.eligible_type_list_id)),
            ..
        }]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// System dbuff emitters
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(in crate::sde) struct SItemBuffSe {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(default)]
    dbuffs: Vec<SItemBuffEntry>,
}
impl ExtractOne<rc::ed::EItemBuff> for SItemBuffSe {
    fn extract(self) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(self.item_id),
            system_emitter_buffs: Some(into_item_buff_data(self.dbuffs, None)),
            ..
        }]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Applied proximity effects
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(in crate::sde) struct SItemBuffPe {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(default)]
    dbuffs: Vec<SItemBuffEntry>,
}
impl ExtractOne<rc::ed::EItemBuff> for SItemBuffPe {
    fn extract(self) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(self.item_id),
            proxy_effect_buffs: Some(into_item_buff_data(self.dbuffs, None)),
            ..
        }]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Proximity traps
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(in crate::sde) struct SItemBuffPt {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(default)]
    dbuffs: Vec<SItemBuffEntry>,
    #[serde(rename = "triggerFilterTypeListID")]
    trigger_filter_type_list_id: Option<i32>,
}
impl ExtractOne<rc::ed::EItemBuff> for SItemBuffPt {
    fn extract(self) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(self.item_id),
            proxy_trigger_buffs: Some(into_item_buff_data(self.dbuffs, self.trigger_filter_type_list_id)),
            ..
        }]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Link-with-ship
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(in crate::sde) struct SItemBuffSl {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(default)]
    dbuffs: Vec<SItemBuffEntry>,
    #[serde(rename = "linkableShipTypeListID")]
    linkable_ship_type_list_id: Option<i32>,
}
impl ExtractOne<rc::ed::EItemBuff> for SItemBuffSl {
    fn extract(self) -> Vec<rc::ed::EItemBuff> {
        vec![rc::ed::EItemBuff {
            item_id: rc::ed::EItemId::from_i32(self.item_id),
            ship_link_buffs: Some(into_item_buff_data(self.dbuffs, self.linkable_ship_type_list_id)),
            ..
        }]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
struct SItemBuffEntry {
    #[serde(rename = "_key")]
    buff_id: i32,
    #[serde(rename = "_value")]
    buff_str: f64,
}

fn into_item_buff_data(buffs: Vec<SItemBuffEntry>, item_list_id: Option<i32>) -> rc::ed::EItemBuffData {
    rc::ed::EItemBuffData {
        buffs: buffs
            .into_iter()
            .map(|entry| rc::ed::EItemBuffEntry {
                id: rc::ed::EBuffId::from_i32(entry.buff_id),
                value: rc::ed::EFloat::from_f64(entry.buff_str),
            })
            .collect(),
        item_list_filter: item_list_id.map(rc::ed::EItemListId::from_i32),
    }
}
