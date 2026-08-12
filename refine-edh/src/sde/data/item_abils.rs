use serde::Deserialize;

use crate::{
    sde::data::ExtractOne,
    shared::data::{ItemAbilData, into_e_item_abils},
};

#[derive(Deserialize)]
pub(in crate::sde) struct SItemAbils {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(rename = "abilitySlot0")]
    ability_slot0: Option<ItemAbilData>,
    #[serde(rename = "abilitySlot1")]
    ability_slot1: Option<ItemAbilData>,
    #[serde(rename = "abilitySlot2")]
    ability_slot2: Option<ItemAbilData>,
}
impl ExtractOne<rc::ed::EItemAbil> for SItemAbils {
    fn extract(self, extracted: &mut Vec<rc::ed::EItemAbil>) {
        extracted.extend(into_e_item_abils(
            self.item_id,
            [self.ability_slot0, self.ability_slot1, self.ability_slot2],
        ));
    }
}
