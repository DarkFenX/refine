use std::{collections::HashSet, hash::Hash};

use crate::{
    consts::{ModAfeeFilter, ModDomain},
    defs::{ReeId, ReeInt},
    ss::item::{SsItem, SsItems},
    util::KeyedStorage1L,
};

use super::affector::AffectorSpec;

pub(in crate::ss::svc) struct AffectionRegister {
    // All known affectee items
    // Contains: HashSet<affectee item IDs>
    afees: HashSet<ReeId>,
    // Top-level items which are representing an "owner" of domain (char, ship)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_topdom: KeyedStorage1L<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit and domain (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_pardom: KeyedStorage1L<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affectee item IDs>
    afees_pardom_grp: KeyedStorage1L<(ReeId, ModDomain, ReeInt), ReeId>,
    // Items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affectee item IDs>
    afees_pardom_srq: KeyedStorage1L<(ReeId, ModDomain, ReeInt), ReeId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    afees_own_srq: KeyedStorage1L<(ReeId, ReeInt), ReeId>,
    // Affector specs registered for an item
    // Contains: KeyedStorage<affector item ID, affector specs>
    afors: KeyedStorage1L<ReeId, AffectorSpec>,
    // Affector specs which modify item directly
    // Contains: KeyedStorage<affectee item ID, affector specs>
    afors_direct: KeyedStorage1L<ReeId, AffectorSpec>,
    // All affector specs which affect top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_topdom: KeyedStorage1L<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs with modifiers which affect 'other' location are always
    // stored here, regardless if they actually affect something or not
    // Contains: KeyedStorage<affector item ID, affector specs>
    afors_other: KeyedStorage1L<ReeId, AffectorSpec>,
    // Affector specs influencing all items belonging to certain fit and domain
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_pardom: KeyedStorage1L<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affector specs>
    afors_pardom_grp: KeyedStorage1L<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affector specs>
    afors_pardom_srq: KeyedStorage1L<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affector specs>
    afors_own_srq: KeyedStorage1L<(ReeId, ReeInt), AffectorSpec>,
}
impl AffectionRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            afees: HashSet::new(),
            afees_topdom: KeyedStorage1L::new(),
            afees_pardom: KeyedStorage1L::new(),
            afees_pardom_grp: KeyedStorage1L::new(),
            afees_pardom_srq: KeyedStorage1L::new(),
            afees_own_srq: KeyedStorage1L::new(),
            afors: KeyedStorage1L::new(),
            afors_direct: KeyedStorage1L::new(),
            afors_topdom: KeyedStorage1L::new(),
            afors_other: KeyedStorage1L::new(),
            afors_pardom: KeyedStorage1L::new(),
            afors_pardom_grp: KeyedStorage1L::new(),
            afors_pardom_srq: KeyedStorage1L::new(),
            afors_own_srq: KeyedStorage1L::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::calc) fn get_local_afee_items(
        &self,
        afor_spec: &AffectorSpec,
        items: &SsItems,
    ) -> Vec<ReeId> {
        let mut afees = Vec::new();
        let afor_item = match items.get_item(&afor_spec.item_id) {
            Ok(i) => i,
            _ => return afees,
        };
        let afor_fit_id = afor_item.get_fit_id();
        match (afor_spec.modifier.afee_filter, afor_fit_id) {
            (ModAfeeFilter::Direct(d), _) => match (d, afor_fit_id) {
                (ModDomain::Item, _) => afees.push(afor_spec.item_id),
                (ModDomain::Char, Some(fid)) => {
                    extend_vec_from_storage(&mut afees, &self.afees_topdom, &(fid, ModDomain::Char))
                }
                (ModDomain::Ship, Some(fid)) => {
                    extend_vec_from_storage(&mut afees, &self.afees_topdom, &(fid, ModDomain::Ship))
                }
                (ModDomain::Other, _) => match afor_item.get_other() {
                    Some(oid) => afees.push(oid),
                    _ => (),
                },
                _ => (),
            },
            (ModAfeeFilter::Loc(d), Some(fid)) => extend_vec_from_storage(&mut afees, &self.afees_pardom, &(fid, d)),
            (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_pardom_grp, &(fid, d, gid))
            }
            (ModAfeeFilter::LocSrq(d, sid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_pardom_srq, &(fid, d, sid))
            }
            (ModAfeeFilter::OwnSrq(_, sid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_own_srq, &(fid, sid))
            }
            _ => (),
        }
        afees
    }
    pub(in crate::ss::svc::calc) fn get_projected_afee_items(&self, afor_spec: ReeId, tgt_items: ReeId) {}
    pub(in crate::ss::svc::calc) fn get_afor_specs_by_afee(&self, afee_item: &SsItem) -> Vec<AffectorSpec> {
        let afee_item_id = afee_item.get_id();
        let afee_fit_id = afee_item.get_fit_id();
        let afee_topdom = afee_item.get_top_domain();
        let afee_pardom = afee_item.get_parent_domain();
        let afee_grp_id = afee_item.get_group_id();
        let afee_srqs = afee_item.get_skill_reqs();
        let mut afors = Vec::new();
        extend_vec_from_storage(&mut afors, &self.afors_direct, &afee_item_id);
        match (afee_fit_id, afee_topdom) {
            (Some(fid), Some(td)) => extend_vec_from_storage(&mut afors, &self.afors_topdom, &(fid, td)),
            _ => (),
        }
        match afee_item.get_other() {
            Some(o) => extend_vec_from_storage(&mut afors, &self.afors_other, &o),
            _ => (),
        }
        match (afee_fit_id, afee_pardom) {
            (Some(fid), Some(pd)) => extend_vec_from_storage(&mut afors, &self.afors_pardom, &(fid, pd)),
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_grp_id) {
            (Some(fid), Some(pd), Ok(gid)) => {
                extend_vec_from_storage(&mut afors, &self.afors_pardom_grp, &(fid, pd, gid));
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, &afee_srqs) {
            (Some(fid), Some(pd), Ok(srqs)) => {
                for skill_a_item_id in srqs.keys() {
                    extend_vec_from_storage(&mut afors, &self.afors_pardom_srq, &(fid, pd, *skill_a_item_id));
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, &afee_srqs) {
                (Some(fid), Ok(srqs)) => {
                    for skill_a_item_id in srqs.keys() {
                        extend_vec_from_storage(&mut afors, &self.afors_own_srq, &(fid, *skill_a_item_id));
                    }
                }
                _ => (),
            }
        }
        afors
    }
    pub(in crate::ss::svc::calc) fn get_afor_specs_by_afor(&self, afor_item_id: &ReeId) -> Vec<AffectorSpec> {
        self.afors
            .get(afor_item_id)
            .map(|v| v.iter().map(|v| v.clone()).collect())
            .unwrap_or_else(|| Vec::new())
    }
    // Maintenance methods
    pub(in crate::ss::svc::calc) fn reg_afee(&mut self, afee_item: &SsItem) {
        let afee_item_id = afee_item.get_id();
        let afee_fit_id = afee_item.get_fit_id();
        let afee_topdom = afee_item.get_top_domain();
        let afee_pardom = afee_item.get_parent_domain();
        let afee_grp_id = afee_item.get_group_id();
        let afee_srqs = afee_item.get_skill_reqs();
        self.afees.insert(afee_item_id);
        match (afee_fit_id, afee_topdom) {
            (Some(fid), Some(td)) => self.afees_topdom.add((fid, td), afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom) {
            (Some(fid), Some(pd)) => self.afees_pardom.add((fid, pd), afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_grp_id) {
            (Some(fid), Some(pd), Ok(gid)) => {
                self.afees_pardom_grp.add((fid, pd, gid), afee_item_id);
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, &afee_srqs) {
            (Some(fid), Some(pd), Ok(srqs)) => {
                for skill_a_item_id in srqs.keys() {
                    self.afees_pardom_srq.add((fid, pd, *skill_a_item_id), afee_item_id);
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, &afee_srqs) {
                (Some(fid), Ok(srqs)) => {
                    for skill_a_item_id in srqs.keys() {
                        self.afees_own_srq.add((fid, *skill_a_item_id), afee_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::ss::svc::calc) fn unreg_afee(&mut self, afee_item: &SsItem) {
        let afee_item_id = afee_item.get_id();
        let afee_fit_id = afee_item.get_fit_id();
        let afee_topdom = afee_item.get_top_domain();
        let afee_pardom = afee_item.get_parent_domain();
        let afee_grp_id = afee_item.get_group_id();
        let afee_srqs = afee_item.get_skill_reqs();
        self.afees.insert(afee_item_id);
        match (afee_fit_id, afee_topdom) {
            (Some(fid), Some(td)) => self.afees_topdom.remove(&(fid, td), &afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom) {
            (Some(fid), Some(pd)) => self.afees_pardom.remove(&(fid, pd), &afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_grp_id) {
            (Some(fid), Some(pd), Ok(gid)) => {
                self.afees_pardom_grp.remove(&(fid, pd, gid), &afee_item_id);
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, &afee_srqs) {
            (Some(fid), Some(pd), Ok(srqs)) => {
                for skill_a_item_id in srqs.keys() {
                    self.afees_pardom_srq
                        .remove(&(fid, pd, *skill_a_item_id), &afee_item_id);
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, &afee_srqs) {
                (Some(fid), Ok(srqs)) => {
                    for skill_a_item_id in srqs.keys() {
                        self.afees_own_srq.remove(&(fid, *skill_a_item_id), &afee_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::ss::svc::calc) fn reg_local_afor_specs(
        &mut self,
        afor_fit_id: Option<ReeId>,
        afor_specs: Vec<AffectorSpec>,
    ) {
        for afor_spec in afor_specs {
            self.afors.add(afor_spec.item_id, afor_spec.clone());
            match (afor_spec.modifier.afee_filter, afor_fit_id) {
                (ModAfeeFilter::Direct(d), _) => match (d, afor_fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.add(afor_spec.item_id, afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_topdom.add((fid, ModDomain::Char), afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_topdom.add((fid, ModDomain::Ship), afor_spec),
                    (ModDomain::Other, _) => self.afors_other.add(afor_spec.item_id, afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(d), Some(fid)) => self.afors_pardom.add((fid, d), afor_spec),
                (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => self.afors_pardom_grp.add((fid, d, gid), afor_spec),
                (ModAfeeFilter::LocSrq(d, srq), Some(fid)) => self.afors_pardom_srq.add((fid, d, srq), afor_spec),
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.add((fid, srq), afor_spec),
                _ => (),
            }
        }
    }
    pub(in crate::ss::svc::calc) fn unreg_local_afor_specs(
        &mut self,
        afor_fit_id: Option<ReeId>,
        afor_specs: Vec<AffectorSpec>,
    ) {
        for afor_spec in afor_specs {
            self.afors.remove(&afor_spec.item_id, &afor_spec);
            match (afor_spec.modifier.afee_filter, afor_fit_id) {
                (ModAfeeFilter::Direct(d), _) => match (d, afor_fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.remove(&afor_spec.item_id, &afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_topdom.remove(&(fid, ModDomain::Char), &afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_topdom.remove(&(fid, ModDomain::Ship), &afor_spec),
                    (ModDomain::Other, _) => self.afors_other.remove(&afor_spec.item_id, &afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(d), Some(fid)) => self.afors_pardom.remove(&(fid, d), &afor_spec),
                (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => self.afors_pardom_grp.remove(&(fid, d, gid), &afor_spec),
                (ModAfeeFilter::LocSrq(d, srq), Some(fid)) => self.afors_pardom_srq.remove(&(fid, d, srq), &afor_spec),
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.remove(&(fid, srq), &afor_spec),
                _ => (),
            }
        }
    }
}

fn extend_vec_from_storage<K: Eq + Hash, V: Eq + Hash + Clone>(
    vec: &mut Vec<V>,
    storage: &KeyedStorage1L<K, V>,
    key: &K,
) {
    match storage.get(key) {
        Some(v) => vec.extend(v.iter().map(|v| v.clone())),
        _ => (),
    }
}
