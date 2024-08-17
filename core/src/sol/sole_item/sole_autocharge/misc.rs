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
    pub(in crate::sol) fn update_item_autocharges(&mut self, item_id: &SolItemId) -> Result<(), ItemAllocError> {
        let item = self.items.get_item(&item_id).unwrap();
        let item_state = item.get_state();
        let mut new_ac_map = StMap::new();
        if let (Some(fit_id), Ok(a_item), Some(_)) = (item.get_fit_id(), item.get_a_item(), item.get_autocharges()) {
            let a_item = a_item.clone();
            for effect_id in a_item.effect_datas.keys() {
                if let Some(effect) = self.src.get_a_effect(effect_id) {
                    if let Some(charge_info) = effect.charge {
                        if let ad::AEffectChargeInfo::Attr(charge_attr_id) = charge_info {
                            if let Some(autocharge_type_id) = a_item.attr_vals.get(&charge_attr_id) {
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
                                let autocharge = SolAutocharge::new(
                                    &self.src,
                                    autocharge_id,
                                    fit_id,
                                    *autocharge_type_id as EItemId,
                                    *item_id,
                                    item_state,
                                    false,
                                );
                                // Don't add an autocharge if it can't be loaded
                                if !autocharge.is_loaded() {
                                    continue;
                                }
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
            item_acs.clear();
            for (effect_id, autocharge_id) in new_ac_map.into_iter() {
                item_acs.set(effect_id, autocharge_id);
            }
        };
        Ok(())
    }
}
