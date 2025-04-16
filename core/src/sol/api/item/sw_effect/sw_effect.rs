use crate::sol::{ItemId, ItemKey, ItemTypeId, SolarSystem, uad::item::UadSwEffect};

pub struct SwEffect<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> SwEffect<'a> {
    pub(in crate::sol) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

pub struct SwEffectMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> SwEffectMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_sw_effect(sol, item_key).get_a_item_id()
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    get_uad_sw_effect(sol, item_key).get_sw_effect_state()
}
fn get_uad_sw_effect(sol: &SolarSystem, item_key: ItemKey) -> &UadSwEffect {
    sol.uad.items.get(item_key).get_sw_effect().unwrap()
}
