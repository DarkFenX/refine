use std::hash::{BuildHasher, Hash};

use crate::{
    ad,
    misc::{AttrSpec, EffectSpec},
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
        a_attr_id: &ad::AAttrId,
        fits: &UFits,
    ) -> Vec<CtxModifier> {
        let fit_key = item.get_fit_key();
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_group_id().unwrap();
        let a_srqs = item.get_skill_reqs().unwrap();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.cmods_direct, item_key, a_attr_id);
        if let Some(other_item_key) = item.get_other_key() {
            filter_and_extend(&mut mods, &self.cmods_other, &other_item_key, a_attr_id);
        }
        if let Some(fit_key) = fit_key {
            let fit = fits.get(fit_key);
            if let Some(root_loc) = root_loc {
                filter_and_extend(&mut mods, &self.cmods_root, &(fit_key, root_loc), a_attr_id);
            }
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.cmods_loc, &(fit_key, loc_kind), a_attr_id);
                filter_and_extend(
                    &mut mods,
                    &self.cmods_loc_grp,
                    &(fit_key, loc_kind, a_item_grp_id),
                    a_attr_id,
                );
                for srq_a_item_id in a_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.cmods_loc_srq,
                        &(fit_key, loc_kind, *srq_a_item_id),
                        a_attr_id,
                    );
                }
            }
            if item.is_owner_modifiable() {
                for srq_a_item_id in a_srqs.keys() {
                    filter_and_extend(&mut mods, &self.cmods_own_srq, &(fit_key, *srq_a_item_id), a_attr_id);
                }
            }
        }
        mods
    }
    pub(in crate::svc::calc) fn iter_affector_spec_cmods(
        &self,
        affector_aspec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = &CtxModifier> {
        self.cmods_by_aspec.get(affector_aspec)
    }
    pub(in crate::svc::calc) fn get_mods_for_added_root(
        &mut self,
        item_key: UItemKey,
        item: &UItem,
    ) -> Vec<CtxModifier> {
        if let Some(buffable_item_lists) = item.get_item_buff_item_lists_nonempty()
            && let UItem::Ship(u_ship) = item
        {
            self.reg_loc_root_for_fw_buff(item_key, u_ship, buffable_item_lists);
            self.reg_loc_root_for_sw_buff(item_key, u_ship, buffable_item_lists);
        }
        self.reg_loc_root_for_proj(item_key, item);
        self.get_mods_for_changed_root(item)
    }
    pub(in crate::svc::calc) fn get_mods_for_removed_root(
        &mut self,
        item_key: UItemKey,
        item: &UItem,
    ) -> Vec<CtxModifier> {
        let cmods = self.get_mods_for_changed_root(item);
        if let Some(buffable_item_lists) = item.get_item_buff_item_lists_nonempty()
            && let UItem::Ship(u_ship) = item
        {
            self.unreg_loc_root_for_fw_buff(item_key, u_ship, buffable_item_lists);
            self.unreg_loc_root_for_sw_buff(item_key, u_ship, buffable_item_lists);
        }
        self.unreg_loc_root_for_proj(item_key, item);
        cmods
    }
    fn get_mods_for_changed_root(&self, item: &UItem) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        if let (Some(fit_key), Some(loc)) = (item.get_fit_key(), item.get_root_loc_kind()) {
            cmods.extend(self.cmods_loc.get(&(fit_key, loc)));
            for ((st_fit_key, st_loc, _), st_cmods) in self.cmods_loc_grp.iter() {
                if fit_key == *st_fit_key && loc == *st_loc {
                    cmods.extend(st_cmods);
                }
            }
            for ((st_fit_key, st_loc, _), st_cmods) in self.cmods_loc_srq.iter() {
                if fit_key == *st_fit_key && loc == *st_loc {
                    cmods.extend(st_cmods);
                }
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn extract_raw_mods_for_effect(
        &mut self,
        reuse_raw_modifiers: &mut Vec<RawModifier>,
        espec: EffectSpec,
    ) {
        reuse_raw_modifiers.clear();
        if let Some(effect_mods) = self.rmods_all.remove_key(&espec) {
            reuse_raw_modifiers.extend(effect_mods)
        }
    }
}

fn filter_and_extend<K, H1, H2>(
    vec: &mut Vec<CtxModifier>,
    storage: &MapSet<K, CtxModifier, H1, H2>,
    key: &K,
    a_attr_id: &ad::AAttrId,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    vec.extend(
        storage
            .get(key)
            .filter(|v| &v.raw.affectee_attr_id == a_attr_id)
            .copied(),
    )
}
