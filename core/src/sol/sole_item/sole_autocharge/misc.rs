use crate::{
    ad,
    sol::{
        ItemId, ItemTypeId, SolarSystem,
        uad::item::{Autocharge, Item},
    },
    util::HMap,
};

impl SolarSystem {
    pub(in crate::sol) fn add_item_autocharges(&mut self, item_id: &ItemId) {
        let item = self.uad.items.get_item(item_id).unwrap();
        let item_a_state = item.get_a_state();
        let projections = match item.iter_projs() {
            Some(projections) => projections.map(|(i, r)| (*i, *r)).collect(),
            None => Vec::new(),
        };
        let mut new_ac_map = HMap::new();
        if let (Some(fit_id), true, Some(_)) = (item.get_fit_id(), item.is_loaded(), item.get_autocharges()) {
            let cloned_item = item.clone();
            for a_effect_id in cloned_item.get_a_effect_datas().unwrap().keys() {
                if let Some(a_effect) = self.uad.src.get_a_effect(a_effect_id) {
                    if let Some(ad::AEffectChargeInfo::Attr(charge_a_attr_id)) = a_effect.charge {
                        if let Some(autocharge_a_item_id) = cloned_item.get_a_attrs().unwrap().get(&charge_a_attr_id) {
                            let autocharge_id = self.uad.items.alloc_item_id();
                            let mut autocharge = Autocharge::new(
                                &self.uad.src,
                                autocharge_id,
                                autocharge_a_item_id.into_inner() as ItemTypeId,
                                fit_id,
                                *item_id,
                                a_effect.id,
                                item_a_state,
                                false,
                            );
                            // Don't add an autocharge if it can't be loaded
                            if !autocharge.is_loaded() {
                                continue;
                            }
                            // Transfer parent item projections to autocharge
                            for (projectee_item_id, range) in projections.iter() {
                                autocharge.get_projs_mut().add(*projectee_item_id, *range);
                            }
                            // Add autocharge item to user data and fill info map
                            new_ac_map.insert(*a_effect_id, autocharge.get_item_id());
                            let ac_item = Item::Autocharge(autocharge);
                            self.uad.items.add_item(ac_item);
                        }
                    }
                }
            }
        }
        if !new_ac_map.is_empty() {
            let item_acs = self
                .uad
                .items
                .get_item_mut(item_id)
                .unwrap()
                .get_autocharges_mut()
                .unwrap();
            for (a_effect_id, autocharge_id) in new_ac_map.into_iter() {
                item_acs.set(a_effect_id, autocharge_id);
            }
        };
    }
}
