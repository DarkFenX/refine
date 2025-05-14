use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemTypeId, SolarSystem,
        api::{FitMut, ImplantMut},
        uad::item::{UadImplant, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_implant(&mut self, fit_key: FitKey, a_item_id: ad::AItemId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_implant = UadImplant::new(&self.uad.src, item_id, a_item_id, fit_key, true);
        let uad_item = UadItem::Implant(uad_implant);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.implants.insert(item_key);
        self.internal_add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut {
        let item_key = self.sol.internal_add_implant(self.key, type_id);
        ImplantMut::new(self.sol, item_key)
    }
}
