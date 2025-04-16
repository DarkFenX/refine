use crate::sol::{
    FitId, ItemId, ItemKey, ItemTypeId, SolarSystem,
    info::{ItemMutationInfo, ProjInfo},
    uad::item::{MinionState, UadDrone},
};

pub struct Drone<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Drone<'a> {
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
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_mutation(&self) -> Option<ItemMutationInfo> {
        get_mutation(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

pub struct DroneMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> DroneMut<'a> {
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
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_mutation(&self) -> Option<ItemMutationInfo> {
        get_mutation(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_drone(sol, item_key).get_a_item_id()
}
fn get_fit_id(sol: &SolarSystem, item_key: ItemKey) -> FitId {
    let fit_key = get_uad_drone(sol, item_key).get_fit_key();
    sol.uad.fits.id_by_key(fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> MinionState {
    get_uad_drone(sol, item_key).get_drone_state()
}
fn get_mutation(sol: &SolarSystem, item_key: ItemKey) -> Option<ItemMutationInfo> {
    get_uad_drone(sol, item_key).get_mutation_info(&sol.uad.src)
}
fn get_projs(sol: &SolarSystem, item_key: ItemKey) -> Vec<ProjInfo> {
    get_uad_drone(sol, item_key)
        .get_projs()
        .iter()
        .map(|(&projectee_item_key, &range)| ProjInfo {
            item_id: sol.uad.items.id_by_key(projectee_item_key),
            range,
        })
        .collect()
}
fn get_uad_drone(sol: &SolarSystem, item_key: ItemKey) -> &UadDrone {
    sol.uad.items.get(item_key).get_drone().unwrap()
}
