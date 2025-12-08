use std::hash::{BuildHasher, Hash};

use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::RAttrKey,
    svc::calc::{
        CtxModifier, RawModifier,
        registers::standard::{StandardRegister, modifier::iter_loc_act::ActiveLocations},
    },
    ud::{UFits, UItem, UItemKey},
    util::MapSet,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn get_mods_for_affectee(
        &self,
        item_key: &UItemKey,
        item: &UItem,
        attr_key: RAttrKey,
        fits: &UFits,
    ) -> Vec<CtxModifier> {
        let fit_key = item.get_fit_key();
        let root_loc = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        let mut cmods = Vec::new();
        filter_and_extend(&mut cmods, &self.cmods.direct, item_key, attr_key);
        if let Some(other_item_key) = item.get_other_key() {
            filter_and_extend(&mut cmods, &self.cmods.other, &other_item_key, attr_key);
        }
        if let Some(fit_key) = fit_key {
            let fit = fits.get(fit_key);
            if let Some(root_loc) = root_loc {
                filter_and_extend(&mut cmods, &self.cmods.root, &(fit_key, root_loc), attr_key);
            }
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut cmods, &self.cmods.loc, &(fit_key, loc_kind), attr_key);
                filter_and_extend(
                    &mut cmods,
                    &self.cmods.loc_grp,
                    &(fit_key, loc_kind, item_grp_id),
                    attr_key,
                );
                for &srq_type_id in srqs.keys() {
                    filter_and_extend(
                        &mut cmods,
                        &self.cmods.loc_srq,
                        &(fit_key, loc_kind, srq_type_id),
                        attr_key,
                    );
                }
            }
            if item.is_owner_modifiable() {
                for &srq_type_id in srqs.keys() {
                    filter_and_extend(&mut cmods, &self.cmods.own_srq, &(fit_key, srq_type_id), attr_key);
                }
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn iter_affector_spec_cmods(
        &self,
        affector_aspec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = &CtxModifier> {
        self.cmods.by_aspec.get(affector_aspec)
    }
    pub(in crate::svc::calc) fn extract_raw_mods_for_effect(
        &mut self,
        reuse_raw_modifiers: &mut Vec<RawModifier>,
        espec: EffectSpec,
    ) {
        reuse_raw_modifiers.clear();
        reuse_raw_modifiers.extend(self.rmods_all.remove_key(&espec))
    }
}

fn filter_and_extend<K, H1, H2>(
    vec: &mut Vec<CtxModifier>,
    storage: &MapSet<K, CtxModifier, H1, H2>,
    key: &K,
    attr_key: RAttrKey,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    vec.extend(
        storage
            .get(key)
            .filter(|v| v.raw.affectee_attr_key == attr_key)
            .copied(),
    )
}
