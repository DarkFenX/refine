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
        let item_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        Ok(self.internal_get_item(item_uid))
    }
    pub(in crate::api) fn internal_get_item(&self, item_uid: UItemId) -> Item<'_> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(_) => Item::Autocharge(Autocharge::new(self, item_uid)),
            UItem::Booster(_) => Item::Booster(Booster::new(self, item_uid)),
            UItem::Character(_) => Item::Character(Character::new(self, item_uid)),
            UItem::Charge(_) => Item::Charge(Charge::new(self, item_uid)),
            UItem::Drone(_) => Item::Drone(Drone::new(self, item_uid)),
            UItem::Fighter(_) => Item::Fighter(Fighter::new(self, item_uid)),
            UItem::FwEffect(_) => Item::FwEffect(FwEffect::new(self, item_uid)),
            UItem::Implant(_) => Item::Implant(Implant::new(self, item_uid)),
            UItem::Module(_) => Item::Module(Module::new(self, item_uid)),
            UItem::ProjEffect(_) => Item::ProjEffect(ProjEffect::new(self, item_uid)),
            UItem::Rig(_) => Item::Rig(Rig::new(self, item_uid)),
            UItem::Service(_) => Item::Service(Service::new(self, item_uid)),
            UItem::Ship(_) => Item::Ship(Ship::new(self, item_uid)),
            UItem::Skill(_) => Item::Skill(Skill::new(self, item_uid)),
            UItem::Stance(_) => Item::Stance(Stance::new(self, item_uid)),
            UItem::Subsystem(_) => Item::Subsystem(Subsystem::new(self, item_uid)),
            UItem::SwEffect(_) => Item::SwEffect(SwEffect::new(self, item_uid)),
        }
    }
    pub fn get_item_mut(&mut self, item_id: &ItemId) -> Result<ItemMut<'_>, GetItemError> {
        let item_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        Ok(self.internal_get_item_mut(item_uid))
    }
    pub(in crate::api) fn internal_get_item_mut(&mut self, item_uid: UItemId) -> ItemMut<'_> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(_) => ItemMut::Autocharge(AutochargeMut::new(self, item_uid)),
            UItem::Booster(_) => ItemMut::Booster(BoosterMut::new(self, item_uid)),
            UItem::Character(_) => ItemMut::Character(CharacterMut::new(self, item_uid)),
            UItem::Charge(_) => ItemMut::Charge(ChargeMut::new(self, item_uid)),
            UItem::Drone(_) => ItemMut::Drone(DroneMut::new(self, item_uid)),
            UItem::Fighter(_) => ItemMut::Fighter(FighterMut::new(self, item_uid)),
            UItem::FwEffect(_) => ItemMut::FwEffect(FwEffectMut::new(self, item_uid)),
            UItem::Implant(_) => ItemMut::Implant(ImplantMut::new(self, item_uid)),
            UItem::Module(_) => ItemMut::Module(ModuleMut::new(self, item_uid)),
            UItem::ProjEffect(_) => ItemMut::ProjEffect(ProjEffectMut::new(self, item_uid)),
            UItem::Rig(_) => ItemMut::Rig(RigMut::new(self, item_uid)),
            UItem::Service(_) => ItemMut::Service(ServiceMut::new(self, item_uid)),
            UItem::Ship(_) => ItemMut::Ship(ShipMut::new(self, item_uid)),
            UItem::Skill(_) => ItemMut::Skill(SkillMut::new(self, item_uid)),
            UItem::Stance(_) => ItemMut::Stance(StanceMut::new(self, item_uid)),
            UItem::Subsystem(_) => ItemMut::Subsystem(SubsystemMut::new(self, item_uid)),
            UItem::SwEffect(_) => ItemMut::SwEffect(SwEffectMut::new(self, item_uid)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
