use std::hash::Hash;

use crate::{
    defs::{EAttrId, EEffectId, SolItemId},
    sol::{
        svc::calc::{registers::SolStandardRegister, SolAttrSpec, SolCtxModifier, SolRawModifier},
        uad::{fit::SolFits, item::SolItem},
    },
    util::StMapSetL1,
};

use super::SolActiveLocations;

impl SolStandardRegister {
    pub(in crate::sol::svc::calc) fn get_mods_for_affectee(
        &self,
        item: &SolItem,
        attr_id: &EAttrId,
        fits: &SolFits,
    ) -> Vec<SolCtxModifier> {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_kind();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.cmods_direct, &item_id, attr_id);
        if let Some(other_item_id) = item.get_other() {
            filter_and_extend(&mut mods, &self.cmods_other, &other_item_id, attr_id);
        }
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            filter_and_extend(&mut mods, &self.cmods_root, &(fit.id, root_loc), attr_id);
        }
        if let Some(fit) = fit_opt {
            for loc_kind in SolActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.cmods_loc, &(fit.id, loc_kind), attr_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_kind in SolActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.cmods_loc_grp, &(fit.id, loc_kind, grp_id), attr_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_kind in SolActiveLocations::new(item, fit) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.cmods_loc_srq, &(fit.id, loc_kind, *srq_id), attr_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.cmods_own_srq, &(fit.id, *srq_id), attr_id);
                }
            }
        }
        mods
    }
    pub(in crate::sol::svc::calc) fn iter_affector_spec_mods(
        &self,
        affector_attr_spec: &SolAttrSpec,
    ) -> impl ExactSizeIterator<Item = &SolCtxModifier> {
        self.cmods_by_attr_spec.get(affector_attr_spec)
    }
    pub(in crate::sol::svc::calc) fn get_mods_for_changed_root(&mut self, item: &SolItem) -> Vec<SolCtxModifier> {
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
        modifiers: &mut Vec<SolRawModifier>,
        item_id: SolItemId,
        effect_id: EEffectId,
    ) {
        modifiers.clear();
        match self.rmods_nonproj.remove_key(&(item_id, effect_id)) {
            Some(effect_mods) => modifiers.extend(effect_mods),
            None => (),
        }
        match self.rmods_proj.remove_key(&(item_id, effect_id)) {
            Some(effect_mods) => modifiers.extend(effect_mods),
            None => (),
        }
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SolCtxModifier>,
    storage: &StMapSetL1<K, SolCtxModifier>,
    key: &K,
    attr_id: &EAttrId,
) {
    vec.extend(
        storage
            .get(key)
            .filter(|v| &v.raw.affectee_attr_id == attr_id)
            .map(|v| *v),
    )
}
