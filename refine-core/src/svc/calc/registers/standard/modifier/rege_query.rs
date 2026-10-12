use crate::{
    misc::AttrSpec,
    rd::RAttrId,
    svc::calc::{
        CtxModKey,
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
    ) -> Vec<CtxModKey> {
        let fit_uid = item.get_fit_uid();
        let root_loc = item.get_root_loc_kind();
        let item_rib = item.get_r_item_base().unwrap();
        let item_grp_id = item_rib.grp_id;
        let srqs = &item_rib.srqs;
        let mut cmods = Vec::new();
        extend_ckeys(&mut cmods, &self.cmods.direct, &(*item_uid, attr_rid));
        if let Some(other_item_uid) = item.get_other_uid() {
            extend_ckeys(&mut cmods, &self.cmods.other, &(other_item_uid, attr_rid));
        }
        if let Some(fit_uid) = fit_uid {
            let fit = fits.get(fit_uid);
            if let Some(root_loc) = root_loc {
                extend_ckeys(&mut cmods, &self.cmods.root, &(fit_uid, root_loc, attr_rid));
            }
            for loc_kind in ActiveLocations::new(item, fit) {
                extend_ckeys(&mut cmods, &self.cmods.loc, &(fit_uid, loc_kind, attr_rid));
                extend_ckeys(
                    &mut cmods,
                    &self.cmods.loc_grp,
                    &(fit_uid, loc_kind, item_grp_id, attr_rid),
                );
                for &srq_type_aid in srqs.keys() {
                    extend_ckeys(
                        &mut cmods,
                        &self.cmods.loc_srq,
                        &(fit_uid, loc_kind, srq_type_aid, attr_rid),
                    );
                }
            }
            if item.is_owner_modifiable() {
                for &srq_type_aid in srqs.keys() {
                    extend_ckeys(&mut cmods, &self.cmods.own_srq, &(fit_uid, srq_type_aid, attr_rid));
                }
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn iter_affector_spec_ckeys(
        &self,
        affector_aspec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = &CtxModKey> {
        self.cmods.by_aspec.get(affector_aspec)
    }
}

fn extend_ckeys<K>(vec: &mut Vec<CtxModKey>, storage: &RMapRSet<K, CtxModKey>, key: &K)
where
    K: Eq + std::hash::Hash,
{
    vec.extend(storage.get(key).copied())
}
