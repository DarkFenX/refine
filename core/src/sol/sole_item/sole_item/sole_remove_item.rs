use crate::{
    err::basic::{ItemFoundError, ItemKindRemoveError},
    sol::{
        ItemId, ItemKey, RmMode, SolarSystem,
        uad::item::{UadAutocharge, UadItem},
    },
    util::Named,
};

impl SolarSystem {
    pub fn remove_item(&mut self, item_id: &ItemId, pos_mode: RmMode) -> Result<(), RemoveItemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_item_internal(item_key, pos_mode)?)
    }
    pub(in crate::sol) fn remove_item_internal(
        &mut self,
        item_key: ItemKey,
        pos_mode: RmMode,
    ) -> Result<(), ItemKindRemoveError> {
        let item = self.uad.items.get(item_key);
        match item {
            // Autocharge can't be removed no matter what
            UadItem::Autocharge(_) => {
                return Err(ItemKindRemoveError {
                    item_kind: UadAutocharge::get_name(),
                });
            }
            // We unwrap when the only reasons of failure are when item is not found and when item
            // kind mismatches, both of which we already checked
            UadItem::Booster(_) => self.remove_booster_internal(item_key).unwrap(),
            UadItem::Character(_) => self.remove_character_internal(item_key).unwrap(),
            UadItem::Charge(_) => self.remove_charge_internal(item_key).unwrap(),
            UadItem::Drone(_) => self.remove_drone_internal(item_key).unwrap(),
            UadItem::Fighter(_) => self.remove_fighter_internal(item_key).unwrap(),
            UadItem::FwEffect(_) => self.remove_fw_effect_internal(item_key).unwrap(),
            UadItem::Implant(_) => self.remove_implant_internal(item_key).unwrap(),
            UadItem::Module(_) => self.remove_module_internal(item_key, pos_mode).unwrap(),
            UadItem::ProjEffect(_) => self.remove_proj_effect_internal(item_key).unwrap(),
            UadItem::Rig(_) => self.remove_rig_internal(item_key).unwrap(),
            UadItem::Service(_) => self.remove_service_internal(item_key).unwrap(),
            UadItem::Ship(_) => self.remove_ship_internal(item_key).unwrap(),
            UadItem::Skill(_) => self.remove_skill_internal(item_key).unwrap(),
            UadItem::Stance(_) => self.remove_stance_internal(item_key).unwrap(),
            UadItem::Subsystem(_) => self.remove_subsystem_internal(item_key).unwrap(),
            UadItem::SwEffect(_) => self.remove_sw_effect_internal(item_key).unwrap(),
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveItemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    UnremovableAutocharge(#[from] ItemKindRemoveError),
}
