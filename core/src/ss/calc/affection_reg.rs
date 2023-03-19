use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

use crate::{
    consts::{ModAfeeFilter, ModDomain},
    ct,
    ss::item::Item,
    util::KeyedStorage,
    ReeId, ReeInt,
};

use super::affector::AffectorSpec;

pub(in crate::ss::calc) struct AffectionRegister {
    // All known affectee items
    // Contains: HashSet<affectee item IDs>
    afees: HashSet<ReeId>,
    // Top-level items which are representing an "owner" of domain (char, ship)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_topdom: KeyedStorage<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit and domain (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_pardom: KeyedStorage<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affectee item IDs>
    afees_pardom_grp: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    // Items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affectee item IDs>
    afees_pardom_srq: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    afees_own_srq: KeyedStorage<(ReeId, ReeInt), ReeId>,
    // Affector specs which modify item directly
    // Contains: KeyedStorage<affectee item ID, affector specs>
    afors_direct: KeyedStorage<ReeId, AffectorSpec>,
    // All affector specs which affect top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_topdom: KeyedStorage<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs with modifiers which affect 'other' location are always
    // stored here, regardless if they actually affect something or not
    // Contains: KeyedStorage<affector item ID, affector specs>
    afors_other: KeyedStorage<ReeId, AffectorSpec>,
    // Affector specs influencing all items belonging to certain fit and domain
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_pardom: KeyedStorage<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affector specs>
    afors_pardom_grp: KeyedStorage<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affector specs>
    afors_pardom_srq: KeyedStorage<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affector specs>
    afors_own_srq: KeyedStorage<(ReeId, ReeInt), AffectorSpec>,
}
impl AffectionRegister {
    pub(in crate::ss::calc) fn new() -> Self {
        Self {
            afees: HashSet::new(),
            afees_topdom: KeyedStorage::new(),
            afees_pardom: KeyedStorage::new(),
            afees_pardom_grp: KeyedStorage::new(),
            afees_pardom_srq: KeyedStorage::new(),
            afees_own_srq: KeyedStorage::new(),
            afors_direct: KeyedStorage::new(),
            afors_topdom: KeyedStorage::new(),
            afors_other: KeyedStorage::new(),
            afors_pardom: KeyedStorage::new(),
            afors_pardom_grp: KeyedStorage::new(),
            afors_pardom_srq: KeyedStorage::new(),
            afors_own_srq: KeyedStorage::new(),
        }
    }
    // Query methods
    pub(in crate::ss::calc) fn get_local_afee_items(
        &mut self,
        afor_spec: &AffectorSpec,
        items: &HashMap<ReeId, Item>,
    ) -> Option<Vec<ReeId>> {
        let afor_item = match items.get(&afor_spec.item_id) {
            Some(i) => i,
            None => return None,
        };
        let afor_fit_id = afor_item.get_fit_id();
        let afor_mod = match afor_spec.get_modifier() {
            Some(am) => am,
            None => return None,
        };
        let mut afees = Vec::new();
        match (&afor_mod.afee_filter, afor_fit_id) {
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
            (ModAfeeFilter::Loc(d), Some(fid)) => extend_vec_from_storage(&mut afees, &self.afees_pardom, &(fid, *d)),
            (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_pardom_grp, &(fid, *d, *gid))
            }
            (ModAfeeFilter::LocSrq(d, sid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_pardom_srq, &(fid, *d, *sid))
            }
            (ModAfeeFilter::OwnSrq(_, sid), Some(fid)) => {
                extend_vec_from_storage(&mut afees, &self.afees_own_srq, &(fid, *sid))
            }
            _ => (),
        }
        Some(afees)
    }
    pub(in crate::ss::calc) fn get_projected_afee_items(&mut self, afor_spec: ReeId, tgt_items: ReeId) {}
    pub(in crate::ss::calc) fn get_afor_specs(&self, afee_item: &Item) -> Vec<AffectorSpec> {
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
            (Some(fid), Some(pd), Some(gid)) => {
                extend_vec_from_storage(&mut afors, &self.afors_pardom_grp, &(fid, pd, gid));
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_srqs) {
            (Some(fid), Some(pd), Some(srqs)) => {
                for skill_type_id in srqs.keys() {
                    extend_vec_from_storage(&mut afors, &self.afors_pardom_srq, &(fid, pd, *skill_type_id));
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, afee_srqs) {
                (Some(fid), Some(srqs)) => {
                    for skill_type_id in srqs.keys() {
                        extend_vec_from_storage(&mut afors, &self.afors_own_srq, &(fid, *skill_type_id));
                    }
                }
                _ => (),
            }
        }
        afors
    }
    // Maintenance methods
    pub(in crate::ss::calc) fn reg_afee(&mut self, afee_item: &Item) {
        let afee_item_id = afee_item.get_id();
        let afee_fit_id = afee_item.get_fit_id();
        let afee_topdom = afee_item.get_top_domain();
        let afee_pardom = afee_item.get_parent_domain();
        let afee_grp_id = afee_item.get_group_id();
        let afee_srqs = afee_item.get_skill_reqs();
        self.afees.insert(afee_item_id);
        match (afee_fit_id, afee_topdom) {
            (Some(fid), Some(td)) => self.afees_topdom.add_entry((fid, td), afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom) {
            (Some(fid), Some(pd)) => self.afees_pardom.add_entry((fid, pd), afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_grp_id) {
            (Some(fid), Some(pd), Some(gid)) => {
                self.afees_pardom_grp.add_entry((fid, pd, gid), afee_item_id);
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_srqs) {
            (Some(fid), Some(pd), Some(srqs)) => {
                for skill_type_id in srqs.keys() {
                    self.afees_pardom_srq.add_entry((fid, pd, *skill_type_id), afee_item_id);
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, afee_srqs) {
                (Some(fid), Some(srqs)) => {
                    for skill_type_id in srqs.keys() {
                        self.afees_own_srq.add_entry((fid, *skill_type_id), afee_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::ss::calc) fn unreg_afee(&mut self, afee_item: &Item) {
        let afee_item_id = afee_item.get_id();
        let afee_fit_id = afee_item.get_fit_id();
        let afee_topdom = afee_item.get_top_domain();
        let afee_pardom = afee_item.get_parent_domain();
        let afee_grp_id = afee_item.get_group_id();
        let afee_srqs = afee_item.get_skill_reqs();
        self.afees.insert(afee_item_id);
        match (afee_fit_id, afee_topdom) {
            (Some(fid), Some(td)) => self.afees_topdom.rm_entry(&(fid, td), &afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom) {
            (Some(fid), Some(pd)) => self.afees_pardom.rm_entry(&(fid, pd), &afee_item_id),
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_grp_id) {
            (Some(fid), Some(pd), Some(gid)) => {
                self.afees_pardom_grp.rm_entry(&(fid, pd, gid), &afee_item_id);
            }
            _ => (),
        }
        match (afee_fit_id, afee_pardom, afee_srqs) {
            (Some(fid), Some(pd), Some(srqs)) => {
                for skill_type_id in srqs.keys() {
                    self.afees_pardom_srq
                        .rm_entry(&(fid, pd, *skill_type_id), &afee_item_id);
                }
            }
            _ => (),
        }
        if afee_item.is_owner_modifiable() {
            match (afee_fit_id, afee_srqs) {
                (Some(fid), Some(srqs)) => {
                    for skill_type_id in srqs.keys() {
                        self.afees_own_srq.rm_entry(&(fid, *skill_type_id), &afee_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::ss::calc) fn reg_local_effect(&mut self, afor_item: &Item, effect: &Arc<ct::Effect>) {
        for (i, afor_mod) in effect.mods.iter().enumerate() {
            let afor_item_id = afor_item.get_id();
            let afor_fit_id = afor_item.get_fit_id();
            let afor_spec = AffectorSpec::new(afor_item_id, effect.clone(), i);
            match (&afor_mod.afee_filter, afor_fit_id) {
                (ModAfeeFilter::Direct(d), _) => match (d, afor_fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.add_entry(afor_item_id, afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_topdom.add_entry((fid, ModDomain::Char), afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_topdom.add_entry((fid, ModDomain::Ship), afor_spec),
                    (ModDomain::Other, _) => self.afors_other.add_entry(afor_item_id, afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(d), Some(fid)) => self.afors_pardom.add_entry((fid, *d), afor_spec),
                (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => {
                    self.afors_pardom_grp.add_entry((fid, *d, *gid), afor_spec)
                }
                (ModAfeeFilter::LocSrq(d, srq), Some(fid)) => {
                    self.afors_pardom_srq.add_entry((fid, *d, *srq), afor_spec)
                }
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.add_entry((fid, *srq), afor_spec),
                _ => (),
            }
        }
    }
    pub(in crate::ss::calc) fn unreg_local_effect(&mut self, afor_item: &Item, effect: &Arc<ct::Effect>) {
        for (i, afor_mod) in effect.mods.iter().enumerate() {
            let afor_item_id = afor_item.get_id();
            let afor_fit_id = afor_item.get_fit_id();
            let afor_spec = AffectorSpec::new(afor_item_id, effect.clone(), i);
            match (&afor_mod.afee_filter, afor_fit_id) {
                (ModAfeeFilter::Direct(d), _) => match (d, afor_fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.rm_entry(&afor_item_id, &afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_topdom.rm_entry(&(fid, ModDomain::Char), &afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_topdom.rm_entry(&(fid, ModDomain::Ship), &afor_spec),
                    (ModDomain::Other, _) => self.afors_other.rm_entry(&afor_item_id, &afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(d), Some(fid)) => self.afors_pardom.rm_entry(&(fid, *d), &afor_spec),
                (ModAfeeFilter::LocGrp(d, gid), Some(fid)) => {
                    self.afors_pardom_grp.rm_entry(&(fid, *d, *gid), &afor_spec)
                }
                (ModAfeeFilter::LocSrq(d, srq), Some(fid)) => {
                    self.afors_pardom_srq.rm_entry(&(fid, *d, *srq), &afor_spec)
                }
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.rm_entry(&(fid, *srq), &afor_spec),
                _ => (),
            }
        }
    }
}

fn extend_vec_from_storage<K, V>(vec: &mut Vec<V>, storage: &KeyedStorage<K, V>, key: &K)
where
    K: Eq + Hash,
    V: Eq + Hash + Clone,
{
    match storage.get(key) {
        Some(v) => vec.extend(v.iter().map(|v| v.clone())),
        _ => (),
    }
}
