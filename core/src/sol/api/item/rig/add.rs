use crate::sol::{
    FitKey, ItemKey, ItemTypeId, SolarSystem,
    api::{FitMut, RigMut},
    uad::item::{UadItem, UadRig},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_rig(&mut self, fit_key: FitKey, type_id: ItemTypeId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_rig = UadRig::new(&self.uad.src, item_id, type_id, fit_key, true);
        let uad_item = UadItem::Rig(uad_rig);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.rigs.insert(item_key);
        self.internal_add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_rig(&mut self, type_id: ItemTypeId) -> RigMut {
        let item_key = self.sol.internal_add_rig(self.key, type_id);
        RigMut::new(self.sol, item_key)
    }
}
