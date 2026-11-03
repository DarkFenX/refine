use crate::{
    api::{
        Autocharge, Booster, Character, Charge, Drone, Fighter, FwEffect, Implant, Item, Module, ProjEffect, Rig,
        Service, Ship, Skill, Stance, Subsystem, SwEffect,
    },
    err::basic::ItemFoundError,
    sol::SolarSystem,
    ud::{ItemId, UItem, UItemId},
};

impl SolarSystem {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<Item<'_>, ItemGetError> {
        let item_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        Ok(self.internal_get_item(item_uid))
    }
    pub(in crate::api::item::item) fn internal_get_item(&mut self, item_uid: UItemId) -> Item<'_> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(..) => Item::Autocharge(Autocharge::new(self, item_uid)),
            UItem::Booster(..) => Item::Booster(Booster::new(self, item_uid)),
            UItem::Character(..) => Item::Character(Character::new(self, item_uid)),
            UItem::Charge(..) => Item::Charge(Charge::new(self, item_uid)),
            UItem::Drone(..) => Item::Drone(Drone::new(self, item_uid)),
            UItem::Fighter(..) => Item::Fighter(Fighter::new(self, item_uid)),
            UItem::FwEffect(..) => Item::FwEffect(FwEffect::new(self, item_uid)),
            UItem::Implant(..) => Item::Implant(Implant::new(self, item_uid)),
            UItem::Module(..) => Item::Module(Module::new(self, item_uid)),
            UItem::ProjEffect(..) => Item::ProjEffect(ProjEffect::new(self, item_uid)),
            UItem::Rig(..) => Item::Rig(Rig::new(self, item_uid)),
            UItem::Service(..) => Item::Service(Service::new(self, item_uid)),
            UItem::Ship(..) => Item::Ship(Ship::new(self, item_uid)),
            UItem::Skill(..) => Item::Skill(Skill::new(self, item_uid)),
            UItem::Stance(..) => Item::Stance(Stance::new(self, item_uid)),
            UItem::Subsystem(..) => Item::Subsystem(Subsystem::new(self, item_uid)),
            UItem::SwEffect(..) => Item::SwEffect(SwEffect::new(self, item_uid)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ItemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
}
