use serde::Deserialize;

use crate::{
    phb::data::{Key, KeyMergeOne},
    shared::data::{ItemAbilData, into_e_item_abils},
};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemAbils {
    #[serde(rename = "abilitySlot0")]
    ability_slot0: Option<ItemAbilData>,
    #[serde(rename = "abilitySlot1")]
    ability_slot1: Option<ItemAbilData>,
    #[serde(rename = "abilitySlot2")]
    ability_slot2: Option<ItemAbilData>,
}
impl KeyMergeOne<rc::ed::EItemAbil> for PItemAbils {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemAbil> {
        into_e_item_abils(key, [self.ability_slot0, self.ability_slot1, self.ability_slot2])
    }
}
