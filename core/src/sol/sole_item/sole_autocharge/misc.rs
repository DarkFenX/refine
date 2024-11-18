use crate::{
    ad,
    defs::{EItemId, SolItemId},
    err::basic::ItemAllocError,
    sol::{
        item::{SolAutocharge, SolItem},
        SolarSystem,
    },
    util::StMap,
};

impl SolarSystem {
    pub(in crate::sol) fn add_item_autocharges(&mut self, item_id: &SolItemId) -> Result<(), ItemAllocError> {
        let item = self.items.get_item(&item_id).unwrap();
        let item_state = item.get_state();
        let projections = match item.iter_projs() {
            Some(projections) => projections.map(|(i, r)| (*i, *r)).collect(),
            None => Vec::new(),
        };
        let mut new_ac_map = StMap::new();
        if let (Some(fit_id), true, Some(_)) = (item.get_fit_id(), item.is_loaded(), item.get_autocharges()) {
            let cloned_item = item.clone();
            for effect_id in cloned_item.get_effect_datas().unwrap().keys() {
                if let Some(effect) = self.src.get_a_effect(effect_id) {
                    if let Some(charge_info) = effect.charge {
                        if let ad::AEffectChargeInfo::Attr(charge_attr_id) = charge_info {
                            if let Some(autocharge_type_id) = cloned_item.get_attrs().unwrap().get(&charge_attr_id) {
                                let autocharge_id = match self.items.alloc_item_id() {
                                    Ok(item_id) => item_id,
                                    Err(e) => {
                                        // If we got an allocation error, remove autocharges we
                                        // already added
                                        for ac_item_id in new_ac_map.values() {
                                            self.items.remove_item(ac_item_id).unwrap();
                                        }
                                        return Err(e);
                                    }
                                };
                                let mut autocharge = SolAutocharge::new(
                                    &self.src,
                                    autocharge_id,
                                    *autocharge_type_id as EItemId,
                                    fit_id,
                                    *item_id,
                                    item_state,
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
                                // Add autocharge item to skeleton and fill info map
                                new_ac_map.insert(*effect_id, autocharge.get_id());
                                let ac_item = SolItem::Autocharge(autocharge);
                                self.items.add_item(ac_item);
                            }
                        }
                    }
                }
            }
        }
        if !new_ac_map.is_empty() {
            let item_acs = self
                .items
                .get_item_mut(&item_id)
                .unwrap()
                .get_autocharges_mut()
                .unwrap();
            for (effect_id, autocharge_id) in new_ac_map.into_iter() {
                item_acs.set(effect_id, autocharge_id);
            }
        };
        Ok(())
    }
}
