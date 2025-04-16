use crate::sol::{FitId, ItemId, ItemKey, ItemTypeId, SlotIndex, SolarSystem, uad::item::UadSubsystem};

pub struct Subsystem<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Subsystem<'a> {
    pub(in crate::sol) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

pub struct SubsystemMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> SubsystemMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_subsystem(sol, item_key).get_a_item_id()
}
fn get_fit_id(sol: &SolarSystem, item_key: ItemKey) -> FitId {
    let fit_key = get_uad_subsystem(sol, item_key).get_fit_key();
    sol.uad.fits.id_by_key(fit_key)
}
fn get_slot(sol: &SolarSystem, item_key: ItemKey) -> Option<SlotIndex> {
    get_uad_subsystem(sol, item_key).get_a_slot()
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    get_uad_subsystem(sol, item_key).get_subsystem_state()
}
fn get_uad_subsystem(sol: &SolarSystem, item_key: ItemKey) -> &UadSubsystem {
    sol.uad.items.get(item_key).get_subsystem().unwrap()
}
