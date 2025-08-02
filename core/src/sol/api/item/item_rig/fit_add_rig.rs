use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, RigMut},
    },
    ud::{UEffectUpdates, UFitKey, UItem, UItemKey, URig},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_rig(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_rig = URig::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::Rig(u_rig);
        let item_key = self.u_data.items.add(u_item);
        u_fit.rigs.insert(item_key);
        SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_rig(&mut self, type_id: ItemTypeId) -> RigMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_add_rig(self.key, type_id, &mut reuse_eupdates);
        RigMut::new(self.sol, item_key)
    }
}
