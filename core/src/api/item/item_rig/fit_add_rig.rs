use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, RigMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, URig},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_rig(
        &mut self,
        fit_uid: UFitId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_rig = URig::new(item_id, type_id, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Rig(u_rig);
        let rig_uid = self.u_data.items.add(u_item);
        u_fit.rigs.insert(rig_uid);
        SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, rig_uid, reuse_eupdates);
        rig_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_rig(&mut self, type_id: ItemTypeId) -> RigMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let rig_uid = self
            .sol
            .internal_add_rig(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        RigMut::new(self.sol, rig_uid)
    }
}
