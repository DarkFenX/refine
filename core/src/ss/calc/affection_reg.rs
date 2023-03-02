use std::collections::HashSet;

use crate::{
    consts::{ModDomain, State},
    ss::item::Item,
    util::KeyedStorage,
    ReeId, ReeInt,
};

pub(in crate::ss::calc) struct AffectionRegister {
    afees: HashSet<ReeId>,
    afees_dom: KeyedStorage<(ReeId, ModDomain), ReeId>,
    afees_dom_grp: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    afees_dom_srq: KeyedStorage<(ReeId, ModDomain, ReeInt), ReeId>,
    afees_own_srq: KeyedStorage<(ReeId, ReeInt), ReeId>,
}
impl AffectionRegister {
    pub(in crate::ss::calc) fn new() -> AffectionRegister {
        AffectionRegister {
            afees: HashSet::new(),
            afees_dom: KeyedStorage::new(),
            afees_dom_grp: KeyedStorage::new(),
            afees_dom_srq: KeyedStorage::new(),
            afees_own_srq: KeyedStorage::new(),
        }
    }
    // Query methods
    pub(in crate::ss::calc) fn get_local_affectee_items(&mut self, affector_spec: ReeId) {}
    pub(in crate::ss::calc) fn get_projected_affectee_items(&mut self, affector_spec: ReeId, tgt_items: ReeId) {}
    pub(in crate::ss::calc) fn get_affector_specs(&mut self, affectee_item: ReeId) {}
    // Maintenance methods
    pub(in crate::ss::calc) fn add_item(&mut self, item: &Item) {
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
    pub(in crate::ss::calc) fn rm_item(&mut self, item: &Item) {
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
    pub(in crate::ss::calc) fn activate_item_state(&mut self, item: &Item, state: State) {}
    pub(in crate::ss::calc) fn deactivate_item_state(&mut self, item: &Item, state: State) {}
}
