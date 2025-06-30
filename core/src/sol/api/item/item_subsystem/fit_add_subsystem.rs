use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    sol::{
        SolarSystem,
        api::{FitMut, SubsystemMut},
    },
    uad::{UadItem, UadSubsystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_subsystem(&mut self, fit_key: FitKey, a_item_id: ad::AItemId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_subsystem = UadSubsystem::new(&self.uad.src, item_id, a_item_id, fit_key, true);
        let uad_item = UadItem::Subsystem(uad_subsystem);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.subsystems.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_subsystem(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut<'_> {
        let item_key = self.sol.internal_add_subsystem(self.key, type_id);
        SubsystemMut::new(self.sol, item_key)
    }
}
