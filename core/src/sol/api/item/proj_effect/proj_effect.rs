use crate::sol::{ItemId, ItemKey, ItemTypeId, SolarSystem, uad::item::UadProjEffect};

pub struct ProjEffect<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ProjEffect<'a> {
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
    pub fn get_projs(&self) -> Vec<ItemId> {
        get_projs(self.sol, self.key)
    }
}

pub struct ProjEffectMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ProjEffectMut<'a> {
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
    pub fn get_projs(&self) -> Vec<ItemId> {
        get_projs(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_proj_effect(sol, item_key).get_a_item_id()
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    get_uad_proj_effect(sol, item_key).get_proj_effect_state()
}
fn get_projs(sol: &SolarSystem, item_key: ItemKey) -> Vec<ItemId> {
    get_uad_proj_effect(sol, item_key)
        .get_projs()
        .iter_projectee_item_keys()
        .map(|&projectee_item_key| sol.uad.items.id_by_key(projectee_item_key))
        .collect()
}
fn get_uad_proj_effect(sol: &SolarSystem, item_key: ItemKey) -> &UadProjEffect {
    sol.uad.items.get(item_key).get_proj_effect().unwrap()
}
