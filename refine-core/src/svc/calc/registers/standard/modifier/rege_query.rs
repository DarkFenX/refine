use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::RAttrId,
    svc::calc::{
        CtxModifier, RawModifier,
        registers::standard::{StandardRegister, modifier::iter_loc_act::ActiveLocations},
    },
    ud::{UFits, UItem, UItemId},
    util::RMapRSet,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn get_mods_for_affectee(
        &self,
        item_uid: &UItemId,
        item: &UItem,
        attr_rid: RAttrId,
        fits: &UFits,
    ) -> Vec<CtxModifier> {
        let fit_uid = item.get_fit_uid();
        let root_loc = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        let mut cmods = Vec::new();
        filter_and_extend(&mut cmods, &self.cmods.direct, item_uid, attr_rid);
        if let Some(other_item_uid) = item.get_other_uid() {
            filter_and_extend(&mut cmods, &self.cmods.other, &other_item_uid, attr_rid);
        }
        if let Some(fit_uid) = fit_uid {
            let fit = fits.get(fit_uid);
            if let Some(root_loc) = root_loc {
                filter_and_extend(&mut cmods, &self.cmods.root, &(fit_uid, root_loc), attr_rid);
            }
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut cmods, &self.cmods.loc, &(fit_uid, loc_kind), attr_rid);
                filter_and_extend(
                    &mut cmods,
                    &self.cmods.loc_grp,
                    &(fit_uid, loc_kind, item_grp_id),
                    attr_rid,
                );
                for &srq_type_aid in srqs.keys() {
                    filter_and_extend(
                        &mut cmods,
                        &self.cmods.loc_srq,
                        &(fit_uid, loc_kind, srq_type_aid),
                        attr_rid,
                    );
                }
            }
            if item.is_owner_modifiable() {
                for &srq_type_aid in srqs.keys() {
                    filter_and_extend(&mut cmods, &self.cmods.own_srq, &(fit_uid, srq_type_aid), attr_rid);
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

fn filter_and_extend<K>(vec: &mut Vec<CtxModifier>, storage: &RMapRSet<K, CtxModifier>, key: &K, attr_rid: RAttrId)
where
    K: Eq + std::hash::Hash,
{
    vec.extend(
        storage
            .get(key)
            .filter(|v| v.raw.affectee_attr_rid == attr_rid)
            .copied(),
    )
}
