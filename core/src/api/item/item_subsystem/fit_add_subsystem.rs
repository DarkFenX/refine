use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, SubsystemMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, USubsystem},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_subsystem(
        &mut self,
        fit_uid: UFitId,
        item_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_subsystem = USubsystem::new(item_id, item_aid, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Subsystem(u_subsystem);
        let subsystem_uid = self.u_data.items.add(u_item);
        u_fit.subsystems.insert(subsystem_uid);
        SolarSystem::util_add_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, reuse_eupdates);
        subsystem_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let subsystem_uid = self
            .sol
            .internal_add_subsystem(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        SubsystemMut::new(self.sol, subsystem_uid)
    }
}
