// Autocharges expose no projection info, since it fully matches projections of the parent item

use crate::sol::{EffectId, FitId, ItemId, ItemKey, ItemTypeId, SolarSystem, uad::item::UadAutocharge};

pub struct Autocharge<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Autocharge<'a> {
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
    pub fn get_cont_item_id(&self) -> ItemId {
        get_cont_item_id(self.sol, self.key)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

pub struct AutochargeMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> AutochargeMut<'a> {
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
    pub fn get_cont_item_id(&self) -> ItemId {
        get_cont_item_id(self.sol, self.key)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_autocharge(sol, item_key).get_a_item_id()
}
fn get_fit_id(sol: &SolarSystem, item_key: ItemKey) -> FitId {
    let fit_key = get_uad_autocharge(sol, item_key).get_fit_key();
    sol.uad.fits.id_by_key(fit_key)
}
fn get_cont_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    let cont_item_key = get_uad_autocharge(sol, item_key).get_cont_item_key();
    sol.uad.items.id_by_key(cont_item_key)
}
fn get_cont_effect_id(sol: &SolarSystem, item_key: ItemKey) -> EffectId {
    get_uad_autocharge(sol, item_key).get_cont_effect_id().into()
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    !get_uad_autocharge(sol, item_key).get_force_disable()
}
fn get_uad_autocharge(sol: &SolarSystem, item_key: ItemKey) -> &UadAutocharge {
    sol.uad.items.get(item_key).get_autocharge().unwrap()
}
