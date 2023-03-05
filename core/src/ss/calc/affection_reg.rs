use std::collections::HashSet;

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
    // Top-level items which are represented by domain (e.g. char, ship)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_top: KeyedStorage<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit and domain (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affectee item IDs>
    afees_dom: KeyedStorage<(ReeId, ModDomain), ReeId>,
    // Items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affectee item IDs>
    afees_dom_grp: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    // Items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affectee item IDs>
    afees_dom_srq: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    afees_own_srq: KeyedStorage<(ReeId, ReeInt), ReeId>,
    // Affector specs which modify item directly
    // Contains: KeyedStorage<affectee item ID, affector specs>
    afors_direct: KeyedStorage<ReeId, AffectorSpec>,
    // All affector specs which affect top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_top: KeyedStorage<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs with modifiers which affect 'other' location are always
    // stored here, regardless if they actually affect something or not
    // Contains: KeyedStorage<affector item ID, affector specs>
    afors_other: KeyedStorage<ReeId, AffectorSpec>,
    // Affector specs influencing all items belonging to certain fit and domain
    // Contains: KeyedStorage<(affectee fit ID, affectee domain), affector specs>
    afors_dom: KeyedStorage<(ReeId, ModDomain), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee group ID), affector specs>
    afors_dom_grp: KeyedStorage<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee domain, affectee skillreq type ID), affector specs>
    afors_dom_srq: KeyedStorage<(ReeId, ModDomain, ReeInt), AffectorSpec>,
    // Affector specs influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(affectee fit ID, affectee skillreq type ID), affector specs>
    afors_own_srq: KeyedStorage<(ReeId, ReeInt), AffectorSpec>,
}
impl AffectionRegister {
    pub(in crate::ss::calc) fn new() -> AffectionRegister {
        AffectionRegister {
            afees: HashSet::new(),
            afees_top: KeyedStorage::new(),
            afees_dom: KeyedStorage::new(),
            afees_dom_grp: KeyedStorage::new(),
            afees_dom_srq: KeyedStorage::new(),
            afees_own_srq: KeyedStorage::new(),
            afors_direct: KeyedStorage::new(),
            afors_top: KeyedStorage::new(),
            afors_other: KeyedStorage::new(),
            afors_dom: KeyedStorage::new(),
            afors_dom_grp: KeyedStorage::new(),
            afors_dom_srq: KeyedStorage::new(),
            afors_own_srq: KeyedStorage::new(),
        }
    }
    // Query methods
    pub(in crate::ss::calc) fn get_local_affectee_items(&mut self, affector_spec: ReeId) {}
    pub(in crate::ss::calc) fn get_projected_affectee_items(&mut self, affector_spec: ReeId, tgt_items: ReeId) {}
    pub(in crate::ss::calc) fn get_affector_specs(&mut self, affectee_item: ReeId) {}
    // Maintenance methods
    pub(in crate::ss::calc) fn reg_afee(&mut self, item: &Item) {
        let item_id = item.get_id();
        self.afees.insert(item_id);
        match item {
            Item::Ship(s) => self.afees_top.add_entry((s.fit_id, ModDomain::Ship), item_id),
            Item::Character(c) => self.afees_top.add_entry((c.fit_id, ModDomain::Char), item_id),
            _ => (),
        }
        match item.get_fit_id() {
            Some(fit_id) => {
                let domain = item.get_parent_domain();
                let group_id = item.get_group_id();
                let skill_reqs = item.get_skill_reqs();
                match domain {
                    Some(d) => {
                        self.afees_dom.add_entry((fit_id, d), item_id);
                        match group_id {
                            Some(gid) => self.afees_dom_grp.add_entry((fit_id, d, gid), item_id),
                            _ => (),
                        };
                        match skill_reqs {
                            Some(srq) => {
                                for skill_id in srq.keys() {
                                    self.afees_dom_srq.add_entry((fit_id, d, *skill_id), item_id)
                                }
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                };
                if item.is_owner_modifiable() {
                    match skill_reqs {
                        Some(srq) => {
                            for skill_id in srq.keys() {
                                self.afees_own_srq.add_entry((fit_id, *skill_id), item_id)
                            }
                        }
                        _ => (),
                    };
                }
            }
            _ => (),
        };
    }
    pub(in crate::ss::calc) fn unreg_afee(&mut self, item: &Item) {
        let item_id = item.get_id();
        self.afees.remove(&item_id);
        match item {
            Item::Ship(s) => self.afees_top.rm_entry((s.fit_id, ModDomain::Ship), &item_id),
            Item::Character(c) => self.afees_top.rm_entry((c.fit_id, ModDomain::Char), &item_id),
            _ => (),
        }
        match item.get_fit_id() {
            Some(fit_id) => {
                let domain = item.get_parent_domain();
                let group_id = item.get_group_id();
                let skill_reqs = item.get_skill_reqs();
                match domain {
                    Some(d) => {
                        self.afees_dom.rm_entry((fit_id, d), &item_id);
                        match group_id {
                            Some(gid) => self.afees_dom_grp.rm_entry((fit_id, d, gid), &item_id),
                            _ => (),
                        };
                        match skill_reqs {
                            Some(srq) => {
                                for skill_id in srq.keys() {
                                    self.afees_dom_srq.rm_entry((fit_id, d, *skill_id), &item_id)
                                }
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                };
                if item.is_owner_modifiable() {
                    match skill_reqs {
                        Some(srq) => {
                            for skill_id in srq.keys() {
                                self.afees_own_srq.rm_entry((fit_id, *skill_id), &item_id)
                            }
                        }
                        _ => (),
                    };
                }
            }
            _ => (),
        };
    }
    pub(in crate::ss::calc) fn reg_local_effect(&mut self, item: &Item, effect: &ct::Effect) {
        for (i, modifier) in effect.mods.iter().enumerate() {
            let item_id = item.get_id();
            let fit_id = item.get_fit_id();
            let afor_spec = AffectorSpec::new(item_id, effect.id, i);
            match (&modifier.afee_filter, fit_id) {
                (ModAfeeFilter::Direct(dom), _) => match (dom, fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.add_entry(item_id, afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_top.add_entry((fid, ModDomain::Char), afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_top.add_entry((fid, ModDomain::Ship), afor_spec),
                    (ModDomain::Other, _) => self.afors_other.add_entry(item_id, afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(dom), Some(fid)) => self.afors_dom.add_entry((fid, *dom), afor_spec),
                (ModAfeeFilter::LocGrp(dom, grp), Some(fid)) => {
                    self.afors_dom_grp.add_entry((fid, *dom, *grp), afor_spec)
                }
                (ModAfeeFilter::LocSrq(dom, srq), Some(fid)) => {
                    self.afors_dom_srq.add_entry((fid, *dom, *srq), afor_spec)
                }
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.add_entry((fid, *srq), afor_spec),
                _ => (),
            }
        }
    }
    pub(in crate::ss::calc) fn unreg_local_effect(&mut self, item: &Item, effect: &ct::Effect) {
        for (i, modifier) in effect.mods.iter().enumerate() {
            let item_id = item.get_id();
            let fit_id = item.get_fit_id();
            let afor_spec = AffectorSpec::new(item_id, effect.id, i);
            match (&modifier.afee_filter, fit_id) {
                (ModAfeeFilter::Direct(dom), _) => match (dom, fit_id) {
                    (ModDomain::Item, _) => self.afors_direct.rm_entry(item_id, &afor_spec),
                    (ModDomain::Char, Some(fid)) => self.afors_top.rm_entry((fid, ModDomain::Char), &afor_spec),
                    (ModDomain::Ship, Some(fid)) => self.afors_top.rm_entry((fid, ModDomain::Ship), &afor_spec),
                    (ModDomain::Other, _) => self.afors_other.rm_entry(item_id, &afor_spec),
                    _ => (),
                },
                (ModAfeeFilter::Loc(dom), Some(fid)) => self.afors_dom.rm_entry((fid, *dom), &afor_spec),
                (ModAfeeFilter::LocGrp(dom, grp), Some(fid)) => {
                    self.afors_dom_grp.rm_entry((fid, *dom, *grp), &afor_spec)
                }
                (ModAfeeFilter::LocSrq(dom, srq), Some(fid)) => {
                    self.afors_dom_srq.rm_entry((fid, *dom, *srq), &afor_spec)
                }
                (ModAfeeFilter::OwnSrq(_, srq), Some(fid)) => self.afors_own_srq.rm_entry((fid, *srq), &afor_spec),
                _ => (),
            }
        }
    }
}
