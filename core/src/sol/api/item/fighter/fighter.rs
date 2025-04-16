use crate::sol::{
    AdjustableCount, FitId, ItemId, ItemKey, ItemTypeId, SolarSystem,
    info::ProjInfo,
    uad::item::{MinionState, UadFighter},
};

pub struct Fighter<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Fighter<'a> {
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
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

pub struct FighterMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> FighterMut<'a> {
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
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_fighter(sol, item_key).get_a_item_id()
}
fn get_fit_id(sol: &SolarSystem, item_key: ItemKey) -> FitId {
    let fit_key = get_uad_fighter(sol, item_key).get_fit_key();
    sol.uad.fits.id_by_key(fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> MinionState {
    get_uad_fighter(sol, item_key).get_fighter_state()
}
fn get_count(sol: &SolarSystem, item_key: ItemKey) -> Option<AdjustableCount> {
    get_uad_fighter(sol, item_key).get_count()
}
fn get_projs(sol: &SolarSystem, item_key: ItemKey) -> Vec<ProjInfo> {
    get_uad_fighter(sol, item_key)
        .get_projs()
        .iter()
        .map(|(&projectee_item_key, &range)| ProjInfo {
            item_id: sol.uad.items.id_by_key(projectee_item_key),
            range,
        })
        .collect()
}
fn get_uad_fighter(sol: &SolarSystem, item_key: ItemKey) -> &UadFighter {
    sol.uad.items.get(item_key).get_fighter().unwrap()
}
