use crate::sol::{
    FitKey, ItemKey, ItemTypeId, SolarSystem,
    api::{FitMut, SubsystemMut},
    uad::item::{UadItem, UadSubsystem},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_subsystem(&mut self, fit_key: FitKey, type_id: ItemTypeId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_subsystem = UadSubsystem::new(&self.uad.src, item_id, type_id, fit_key, true);
        let uad_item = UadItem::Subsystem(uad_subsystem);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.subsystems.insert(item_key);
        self.internal_add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut {
        let item_key = self.sol.internal_add_subsystem(self.key, type_id);
        SubsystemMut::new(self.sol, item_key)
    }
}
