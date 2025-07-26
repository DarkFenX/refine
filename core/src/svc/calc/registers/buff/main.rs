use crate::{
    ad,
    misc::AttrSpec,
    rd::{REffect, REffectKey},
    svc::calc::RawModifier,
    ud::UItemKey,
    util::RMapRSet,
};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
#[derive(Clone)]
pub(in crate::svc::calc) struct BuffRegister {
    pub(super) effect_keys: RMapRSet<UItemKey, REffectKey>,
    pub(super) rmods: RMapRSet<AttrSpec, RawModifier>,
}
impl BuffRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            effect_keys: RMapRSet::new(),
            rmods: RMapRSet::new(),
        }
    }
    // Effect methods
    pub(in crate::svc::calc) fn get_effects(&self, item_key: &UItemKey) -> impl ExactSizeIterator<Item = REffectKey> {
        self.effect_keys.get(item_key).copied()
    }
    pub(in crate::svc::calc) fn reg_effect(&mut self, item_key: UItemKey, effect: &REffect) {
        if uses_default_attrs(effect) {
            self.effect_keys.add_entry(item_key, effect.get_key());
        }
    }
    pub(in crate::svc::calc) fn unreg_effect(&mut self, item_key: UItemKey, effect: &REffect) {
        if uses_default_attrs(effect) {
            self.effect_keys.remove_entry(&item_key, &effect.get_key());
        }
    }
    // Modifier methods
    pub(in crate::svc::calc) fn extract_dependent_mods(
        &mut self,
        aspec: &AttrSpec,
    ) -> Option<impl ExactSizeIterator<Item = RawModifier> + use<>> {
        self.rmods.remove_key(aspec)
    }
    pub(in crate::svc::calc) fn reg_dependent_mod(&mut self, aspec: AttrSpec, rmod: RawModifier) {
        self.rmods.add_entry(aspec, rmod)
    }
    pub(in crate::svc::calc) fn unreg_dependent_mod(&mut self, aspec: &AttrSpec, rmod: &RawModifier) {
        self.rmods.remove_entry(aspec, rmod);
    }
}

fn uses_default_attrs(effect: &REffect) -> bool {
    match &effect.get_buff_info() {
        Some(buff_info) => matches!(buff_info.source, ad::AEffectBuffSrc::DefaultAttrs),
        _ => false,
    }
}
