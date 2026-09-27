use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, RigMut},
    sol::SolarSystem,
    ud::{UFitId, UItem, UItemId, URig},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_rig(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_rig = URig::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Rig(u_rig);
        let rig_uid = self.u_data.items.add(u_item);
        u_fit.rigs.insert(rig_uid);
        SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, rig_uid, &mut self.cache.eupdates);
        rig_uid
    }
}

impl<'s> FitMut<'s> {
    pub fn add_rig(&mut self, type_id: ItemTypeId) -> RigMut<'_> {
        let rig_uid = self.sol.internal_add_rig(self.uid, type_id.into_aid());
        RigMut::new(self.sol, rig_uid)
    }
}
