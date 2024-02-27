use std::collections::HashSet;

use crate::{ss::item::SsItem, util::KeyedStorage1L, ModDomain, SsItemId};

pub(in crate::ss::svc::calc) struct ProjRegister {
    // All directly modifiable items
    // Contains: HashSet<item IDs>
    direct: HashSet<SsItemId>,
    // Items which are owners of locations
    // Contains: KeyedStorage<domain, item IDs>
    loc_owners: KeyedStorage1L<ModDomain, SsItemId>,
}
impl ProjRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            direct: HashSet::new(),
            loc_owners: KeyedStorage1L::new(),
        }
    }
    // Query methods
    // Modification methods
    pub(in crate::ss::svc::calc) fn item_added(&mut self, item: &SsItem) {
        if let Some(dom) = item.get_top_domain() {
            self.loc_owners.add(dom, item.get_id())
        }
        ()
    }
    pub(in crate::ss::svc::calc) fn item_removed(&mut self, item: &SsItem) {
        if let Some(dom) = item.get_top_domain() {
            self.loc_owners.remove(&dom, &item.get_id())
        }
        ()
    }
    pub(in crate::ss::svc::calc) fn item_loaded(&mut self, item: &SsItem) {
        if item.is_directly_modifiable() {
            self.direct.insert(item.get_id());
            ()
        }
    }
    pub(in crate::ss::svc::calc) fn item_unloaded(&mut self, item: &SsItem) {
        if item.is_directly_modifiable() {
            self.direct.remove(&item.get_id());
            ()
        }
    }
}
