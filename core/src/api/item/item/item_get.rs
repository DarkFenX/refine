use crate::{
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, Item, ItemMut, Module, ModuleMut, ProjEffect,
        ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, Skill, SkillMut, Stance, StanceMut, Subsystem,
        SubsystemMut, SwEffect, SwEffectMut,
    },
    def::ItemId,
    err::basic::ItemFoundError,
    sol::SolarSystem,
    ud::{UItem, UItemId},
};

impl SolarSystem {
    pub fn get_item(&self, item_id: &ItemId) -> Result<Item<'_>, GetItemError> {
        let item_key = self.u_data.items.iid_by_eid_err(item_id)?;
        Ok(self.internal_get_item(item_key))
    }
    pub(in crate::api) fn internal_get_item(&self, item_key: UItemId) -> Item<'_> {
        let u_item = self.u_data.items.get(item_key);
        match u_item {
            UItem::Autocharge(_) => Item::Autocharge(Autocharge::new(self, item_key)),
            UItem::Booster(_) => Item::Booster(Booster::new(self, item_key)),
            UItem::Character(_) => Item::Character(Character::new(self, item_key)),
            UItem::Charge(_) => Item::Charge(Charge::new(self, item_key)),
            UItem::Drone(_) => Item::Drone(Drone::new(self, item_key)),
            UItem::Fighter(_) => Item::Fighter(Fighter::new(self, item_key)),
            UItem::FwEffect(_) => Item::FwEffect(FwEffect::new(self, item_key)),
            UItem::Implant(_) => Item::Implant(Implant::new(self, item_key)),
            UItem::Module(_) => Item::Module(Module::new(self, item_key)),
            UItem::ProjEffect(_) => Item::ProjEffect(ProjEffect::new(self, item_key)),
            UItem::Rig(_) => Item::Rig(Rig::new(self, item_key)),
            UItem::Service(_) => Item::Service(Service::new(self, item_key)),
            UItem::Ship(_) => Item::Ship(Ship::new(self, item_key)),
            UItem::Skill(_) => Item::Skill(Skill::new(self, item_key)),
            UItem::Stance(_) => Item::Stance(Stance::new(self, item_key)),
            UItem::Subsystem(_) => Item::Subsystem(Subsystem::new(self, item_key)),
            UItem::SwEffect(_) => Item::SwEffect(SwEffect::new(self, item_key)),
        }
    }
    pub fn get_item_mut(&mut self, item_id: &ItemId) -> Result<ItemMut<'_>, GetItemError> {
        let item_key = self.u_data.items.iid_by_eid_err(item_id)?;
        Ok(self.internal_get_item_mut(item_key))
    }
    pub(in crate::api) fn internal_get_item_mut(&mut self, item_key: UItemId) -> ItemMut<'_> {
        let u_item = self.u_data.items.get(item_key);
        match u_item {
            UItem::Autocharge(_) => ItemMut::Autocharge(AutochargeMut::new(self, item_key)),
            UItem::Booster(_) => ItemMut::Booster(BoosterMut::new(self, item_key)),
            UItem::Character(_) => ItemMut::Character(CharacterMut::new(self, item_key)),
            UItem::Charge(_) => ItemMut::Charge(ChargeMut::new(self, item_key)),
            UItem::Drone(_) => ItemMut::Drone(DroneMut::new(self, item_key)),
            UItem::Fighter(_) => ItemMut::Fighter(FighterMut::new(self, item_key)),
            UItem::FwEffect(_) => ItemMut::FwEffect(FwEffectMut::new(self, item_key)),
            UItem::Implant(_) => ItemMut::Implant(ImplantMut::new(self, item_key)),
            UItem::Module(_) => ItemMut::Module(ModuleMut::new(self, item_key)),
            UItem::ProjEffect(_) => ItemMut::ProjEffect(ProjEffectMut::new(self, item_key)),
            UItem::Rig(_) => ItemMut::Rig(RigMut::new(self, item_key)),
            UItem::Service(_) => ItemMut::Service(ServiceMut::new(self, item_key)),
            UItem::Ship(_) => ItemMut::Ship(ShipMut::new(self, item_key)),
            UItem::Skill(_) => ItemMut::Skill(SkillMut::new(self, item_key)),
            UItem::Stance(_) => ItemMut::Stance(StanceMut::new(self, item_key)),
            UItem::Subsystem(_) => ItemMut::Subsystem(SubsystemMut::new(self, item_key)),
            UItem::SwEffect(_) => ItemMut::SwEffect(SwEffectMut::new(self, item_key)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
