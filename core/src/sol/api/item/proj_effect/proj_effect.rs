use crate::sol::{
    ItemId, ItemKey, SolarSystem,
    api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    uad::item::UadProjEffect,
};

pub struct ProjEffect<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ProjEffect<'a> {
    pub(in crate::sol) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ItemId> {
        get_projs(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ProjEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for ProjEffect<'a> {}

pub struct ProjEffectMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ProjEffectMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ItemId> {
        get_projs(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ProjEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ProjEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ProjEffectMut<'a> {}
impl<'a> ItemMutCommon for ProjEffectMut<'a> {}

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
