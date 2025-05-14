use crate::sol::{
    ItemKey, SolarSystem,
    svc::Svc,
    uad::{Uad, item::UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn load_charge(svc: &mut Svc, uad: &Uad, item_key: ItemKey, uad_item: &UadItem) {
        svc.load_item(uad, item_key, uad_item);
    }
    pub(in crate::sol::api) fn unload_charge(svc: &mut Svc, uad: &Uad, item_key: ItemKey, uad_item: &UadItem) {
        svc.unload_item(uad, item_key, uad_item);
    }
}
