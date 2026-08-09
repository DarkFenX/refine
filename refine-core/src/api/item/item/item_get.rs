use crate::{
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, Item, ItemMut, Module, ModuleMut, ProjEffect,
        ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, Skill, SkillMut, Stance, StanceMut, Subsystem,
        SubsystemMut, SwEffect, SwEffectMut,
    },
    err::basic::ItemFoundError,
    sol::SolarSystem,
    ud::{ItemId, UItem, UItemId},
};

impl SolarSystem {
    pub fn get_item(&self, item_id: &ItemId) -> Result<Item<'_>, GetItemError> {
        let item_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        Ok(self.internal_get_item(item_uid))
    }
    pub(in crate::api) fn internal_get_item(&self, item_uid: UItemId) -> Item<'_> {
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
    pub fn get_item_mut(&mut self, item_id: &ItemId) -> Result<ItemMut<'_>, GetItemError> {
        let item_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        Ok(self.internal_get_item_mut(item_uid))
    }
    pub(in crate::api) fn internal_get_item_mut(&mut self, item_uid: UItemId) -> ItemMut<'_> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(..) => ItemMut::Autocharge(AutochargeMut::new(self, item_uid)),
            UItem::Booster(..) => ItemMut::Booster(BoosterMut::new(self, item_uid)),
            UItem::Character(..) => ItemMut::Character(CharacterMut::new(self, item_uid)),
            UItem::Charge(..) => ItemMut::Charge(ChargeMut::new(self, item_uid)),
            UItem::Drone(..) => ItemMut::Drone(DroneMut::new(self, item_uid)),
            UItem::Fighter(..) => ItemMut::Fighter(FighterMut::new(self, item_uid)),
            UItem::FwEffect(..) => ItemMut::FwEffect(FwEffectMut::new(self, item_uid)),
            UItem::Implant(..) => ItemMut::Implant(ImplantMut::new(self, item_uid)),
            UItem::Module(..) => ItemMut::Module(ModuleMut::new(self, item_uid)),
            UItem::ProjEffect(..) => ItemMut::ProjEffect(ProjEffectMut::new(self, item_uid)),
            UItem::Rig(..) => ItemMut::Rig(RigMut::new(self, item_uid)),
            UItem::Service(..) => ItemMut::Service(ServiceMut::new(self, item_uid)),
            UItem::Ship(..) => ItemMut::Ship(ShipMut::new(self, item_uid)),
            UItem::Skill(..) => ItemMut::Skill(SkillMut::new(self, item_uid)),
            UItem::Stance(..) => ItemMut::Stance(StanceMut::new(self, item_uid)),
            UItem::Subsystem(..) => ItemMut::Subsystem(SubsystemMut::new(self, item_uid)),
            UItem::SwEffect(..) => ItemMut::SwEffect(SwEffectMut::new(self, item_uid)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetItemError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
}
