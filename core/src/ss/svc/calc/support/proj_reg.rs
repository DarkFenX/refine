use std::collections::HashSet;

use crate::ss::{item::SsItem, svc::calc::support::SsAttrMod};

pub(in crate::ss::svc::calc) struct ProjRegister {
    // All system-wide modifiers
    sw_mods: HashSet<SsAttrMod>,
}
impl ProjRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            sw_mods: HashSet::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::calc) fn get_sw_mods(&self) -> &HashSet<SsAttrMod> {
        &self.sw_mods
    }
    // Modification methods
    pub(in crate::ss::svc::calc) fn add_sw_mod(&mut self, sw_mod: SsAttrMod) {
        self.sw_mods.insert(sw_mod);
    }
    pub(in crate::ss::svc::calc) fn remove_sw_mod(&mut self, sw_mod: SsAttrMod) {
        self.sw_mods.remove(&sw_mod);
    }
}
