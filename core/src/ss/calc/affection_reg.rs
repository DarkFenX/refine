use std::collections::HashSet;

use crate::{consts::ModDomain, ss::item::Item, util::KeyedStorage, ReeId, ReeInt};

use super::affector::AffectorSpec;

pub(in crate::ss::calc) struct AffectionRegister {
    // All known affectee items
    // Contains: HashSet<affectee item IDs>
    afees: HashSet<ReeId>,
    // Items belonging to certain fit and domain
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
    // Affector specs with modifiers which affect 'other' location are always
    // stored here, regardless if they actually affect something or not
    // Contains: KeyedStorage<affector item ID, affector specs>
    afors_item_other: KeyedStorage<ReeId, AffectorSpec>,
    // Affector specs which should affect only one item (ship, character or
    // self), when this item is not registered as affectee
    // Contains: KeyedStorage<affectee fit ID, affector specs>
    afors_item_await: KeyedStorage<ReeId, AffectorSpec>,
    // All active affector specs which affect one specific item (via ship,
    // character, other reference or self) are kept here
    // Contains: KeyedStorage<affectee item ID, affector specs>
    afors_item_active: KeyedStorage<ReeId, AffectorSpec>,
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
            afees_dom: KeyedStorage::new(),
            afees_dom_grp: KeyedStorage::new(),
            afees_dom_srq: KeyedStorage::new(),
            afees_own_srq: KeyedStorage::new(),
            afors_item_other: KeyedStorage::new(),
            afors_item_await: KeyedStorage::new(),
            afors_item_active: KeyedStorage::new(),
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
        match item.get_fit_id() {
            Some(fit_id) => {
                let domain = item.get_domain();
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
        match item.get_fit_id() {
            Some(fit_id) => {
                let domain = item.get_domain();
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
    pub(in crate::ss::calc) fn reg_local_afor_spec(&mut self, afor_spec: AffectorSpec) {}
    pub(in crate::ss::calc) fn unreg_local_afor_spec(&mut self, afor_spec: AffectorSpec) {}
}
