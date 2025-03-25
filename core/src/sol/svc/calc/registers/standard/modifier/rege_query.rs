use std::hash::Hash;

use crate::{
    ad,
    sol::{
        ItemId,
        svc::calc::{AttrSpec, CtxModifier, RawModifier, registers::StandardRegister},
        uad::{fit::Fits, item::Item},
    },
    util::StMapSetL1,
};

use super::ActiveLocations;

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn get_mods_for_affectee(
        &self,
        item: &Item,
        a_attr_id: &ad::AAttrId,
        fits: &Fits,
    ) -> Vec<CtxModifier> {
        let item_id = item.get_item_id();
        let fit = item.get_fit_id().and_then(|v| fits.get_fit(&v).ok());
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_a_group_id();
        let a_srqs = item.get_a_skill_reqs();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.cmods_direct, &item_id, a_attr_id);
        if let Some(other_item_id) = item.get_other() {
            filter_and_extend(&mut mods, &self.cmods_other, &other_item_id, a_attr_id);
        }
        if let (Some(fit), Some(root_loc)) = (fit, root_loc) {
            filter_and_extend(&mut mods, &self.cmods_root, &(fit.id, root_loc), a_attr_id);
        }
        if let Some(fit) = fit {
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.cmods_loc, &(fit.id, loc_kind), a_attr_id);
            }
        }
        if let (Some(fit), Some(a_item_grp_id)) = (fit, a_item_grp_id) {
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(
                    &mut mods,
                    &self.cmods_loc_grp,
                    &(fit.id, loc_kind, a_item_grp_id),
                    a_attr_id,
                );
            }
        }
        if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
            for loc_kind in ActiveLocations::new(item, fit) {
                for srq_a_item_id in a_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.cmods_loc_srq,
                        &(fit.id, loc_kind, *srq_a_item_id),
                        a_attr_id,
                    );
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
                for srq_a_item_id in a_srqs.keys() {
                    filter_and_extend(&mut mods, &self.cmods_own_srq, &(fit.id, *srq_a_item_id), a_attr_id);
                }
            }
        }
        mods
    }
    pub(in crate::sol::svc::calc) fn iter_affector_spec_mods(
        &self,
        affector_attr_spec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = &CtxModifier> {
        self.cmods_by_attr_spec.get(affector_attr_spec)
    }
    pub(in crate::sol::svc::calc) fn get_mods_for_changed_root(&mut self, item: &Item) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        if let (Some(fit_id), Some(loc)) = (item.get_fit_id(), item.get_root_loc_kind()) {
            cmods.extend(self.cmods_loc.get(&(fit_id, loc)));
            for ((st_fit_id, st_loc, _), st_cmods) in self.cmods_loc_grp.iter() {
                if fit_id == *st_fit_id && loc == *st_loc {
                    cmods.extend(st_cmods);
                }
            }
            for ((st_fit_id, st_loc, _), st_cmods) in self.cmods_loc_srq.iter() {
                if fit_id == *st_fit_id && loc == *st_loc {
                    cmods.extend(st_cmods);
                }
            }
        }
        cmods
    }
    pub(in crate::sol::svc::calc) fn extract_raw_mods_for_effect(
        &mut self,
        raw_modifiers: &mut Vec<RawModifier>,
        item_id: ItemId,
        a_effect_id: ad::AEffectId,
    ) {
        raw_modifiers.clear();
        if let Some(effect_mods) = self.rmods_nonproj.remove_key(&(item_id, a_effect_id)) {
            raw_modifiers.extend(effect_mods)
        }
        if let Some(effect_mods) = self.rmods_proj.remove_key(&(item_id, a_effect_id)) {
            raw_modifiers.extend(effect_mods)
        }
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<CtxModifier>,
    storage: &StMapSetL1<K, CtxModifier>,
    key: &K,
    a_attr_id: &ad::AAttrId,
) {
    vec.extend(
        storage
            .get(key)
            .filter(|v| &v.raw.affectee_a_attr_id == a_attr_id)
            .copied(),
    )
}
