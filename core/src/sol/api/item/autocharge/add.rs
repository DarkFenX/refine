use crate::{
    ad,
    sol::{
        ItemKey, ItemTypeId, SolarSystem,
        uad::item::{UadAutocharge, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_item_autocharges(&mut self, item_key: ItemKey) {
        let item = self.uad.items.get(item_key);
        let item_a_state = item.get_a_state();
        let projections = match item.iter_projs() {
            Some(projections) => projections
                .map(|(&projectee_item_key, &range)| (projectee_item_key, range))
                .collect(),
            None => Vec::new(),
        };
        let mut new_autocharges = Vec::new();
        if let (Some(fit_id), true, Some(_)) = (item.get_fit_key(), item.is_loaded(), item.get_autocharges()) {
            let cloned_item = item.clone();
            for a_effect_id in cloned_item.get_a_effect_datas().unwrap().keys() {
                if let Some(a_effect) = self.uad.src.get_a_effect(a_effect_id) {
                    if let Some(ad::AEffectChargeInfo::Attr(charge_a_attr_id)) = a_effect.charge {
                        if let Some(autocharge_a_item_id) = cloned_item.get_a_attrs().unwrap().get(&charge_a_attr_id) {
                            let autocharge_id = self.uad.items.alloc_id();
                            let mut autocharge = UadAutocharge::new(
                                &self.uad.src,
                                autocharge_id,
                                autocharge_a_item_id.into_inner() as ItemTypeId,
                                fit_id,
                                item_key,
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
                            let ac_item = UadItem::Autocharge(autocharge);
                            let ac_key = self.uad.items.add(ac_item);
                            new_autocharges.push((*a_effect_id, ac_key));
                        }
                    }
                }
            }
        }
        if !new_autocharges.is_empty() {
            let item_acs = self.uad.items.get_mut(item_key).get_autocharges_mut().unwrap();
            for (a_effect_id, autocharge_key) in new_autocharges.into_iter() {
                item_acs.set(a_effect_id, autocharge_key);
            }
        };
    }
}
