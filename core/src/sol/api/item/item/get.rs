use crate::{
    def::ItemId,
    err::basic::ItemFoundError,
    sol::{
        SolarSystem,
        api::{
            Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone,
            DroneMut, Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, Item, ItemMut, Module,
            ModuleMut, ProjEffect, ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, Skill, SkillMut,
            Stance, StanceMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
        },
    },
    uad::{UadItem, UadItemKey},
};

impl SolarSystem {
    pub fn get_item(&self, item_id: &ItemId) -> Result<Item<'_>, GetItemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.internal_get_item(item_key))
    }
    pub fn internal_get_item(&self, item_key: UadItemKey) -> Item<'_> {
        let uad_item = self.uad.items.get(item_key);
        match uad_item {
            UadItem::Autocharge(_) => Item::Autocharge(Autocharge::new(self, item_key)),
            UadItem::Booster(_) => Item::Booster(Booster::new(self, item_key)),
            UadItem::Character(_) => Item::Character(Character::new(self, item_key)),
            UadItem::Charge(_) => Item::Charge(Charge::new(self, item_key)),
            UadItem::Drone(_) => Item::Drone(Drone::new(self, item_key)),
            UadItem::Fighter(_) => Item::Fighter(Fighter::new(self, item_key)),
            UadItem::FwEffect(_) => Item::FwEffect(FwEffect::new(self, item_key)),
            UadItem::Implant(_) => Item::Implant(Implant::new(self, item_key)),
            UadItem::Module(_) => Item::Module(Module::new(self, item_key)),
            UadItem::ProjEffect(_) => Item::ProjEffect(ProjEffect::new(self, item_key)),
            UadItem::Rig(_) => Item::Rig(Rig::new(self, item_key)),
            UadItem::Service(_) => Item::Service(Service::new(self, item_key)),
            UadItem::Ship(_) => Item::Ship(Ship::new(self, item_key)),
            UadItem::Skill(_) => Item::Skill(Skill::new(self, item_key)),
            UadItem::Stance(_) => Item::Stance(Stance::new(self, item_key)),
            UadItem::Subsystem(_) => Item::Subsystem(Subsystem::new(self, item_key)),
            UadItem::SwEffect(_) => Item::SwEffect(SwEffect::new(self, item_key)),
        }
    }
    pub fn get_item_mut(&mut self, item_id: &ItemId) -> Result<ItemMut<'_>, GetItemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.internal_get_item_mut(item_key))
    }
    pub fn internal_get_item_mut(&mut self, item_key: UadItemKey) -> ItemMut<'_> {
        let uad_item = self.uad.items.get(item_key);
        match uad_item {
            UadItem::Autocharge(_) => ItemMut::Autocharge(AutochargeMut::new(self, item_key)),
            UadItem::Booster(_) => ItemMut::Booster(BoosterMut::new(self, item_key)),
            UadItem::Character(_) => ItemMut::Character(CharacterMut::new(self, item_key)),
            UadItem::Charge(_) => ItemMut::Charge(ChargeMut::new(self, item_key)),
            UadItem::Drone(_) => ItemMut::Drone(DroneMut::new(self, item_key)),
            UadItem::Fighter(_) => ItemMut::Fighter(FighterMut::new(self, item_key)),
            UadItem::FwEffect(_) => ItemMut::FwEffect(FwEffectMut::new(self, item_key)),
            UadItem::Implant(_) => ItemMut::Implant(ImplantMut::new(self, item_key)),
            UadItem::Module(_) => ItemMut::Module(ModuleMut::new(self, item_key)),
            UadItem::ProjEffect(_) => ItemMut::ProjEffect(ProjEffectMut::new(self, item_key)),
            UadItem::Rig(_) => ItemMut::Rig(RigMut::new(self, item_key)),
            UadItem::Service(_) => ItemMut::Service(ServiceMut::new(self, item_key)),
            UadItem::Ship(_) => ItemMut::Ship(ShipMut::new(self, item_key)),
            UadItem::Skill(_) => ItemMut::Skill(SkillMut::new(self, item_key)),
            UadItem::Stance(_) => ItemMut::Stance(StanceMut::new(self, item_key)),
            UadItem::Subsystem(_) => ItemMut::Subsystem(SubsystemMut::new(self, item_key)),
            UadItem::SwEffect(_) => ItemMut::SwEffect(SwEffectMut::new(self, item_key)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
