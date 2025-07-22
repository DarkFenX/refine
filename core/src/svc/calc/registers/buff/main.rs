use crate::{ad, misc::AttrSpec, svc::calc::RawModifier, uad::UadItemKey, util::RMapRSet};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
#[derive(Clone)]
pub(in crate::svc::calc) struct BuffRegister {
    pub(super) a_effect_ids: RMapRSet<UadItemKey, ad::AEffectId>,
    pub(super) rmods: RMapRSet<AttrSpec, RawModifier>,
}
impl BuffRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            a_effect_ids: RMapRSet::new(),
            rmods: RMapRSet::new(),
        }
    }
    // Effect methods
    pub(in crate::svc::calc) fn get_effects(
        &self,
        item_key: &UadItemKey,
    ) -> impl ExactSizeIterator<Item = &ad::AEffectId> {
        self.a_effect_ids.get(item_key)
    }
    pub(in crate::svc::calc) fn reg_effect(&mut self, item_key: UadItemKey, effect: &ad::AEffectRt) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.add_entry(item_key, effect.ae.id);
        }
    }
    pub(in crate::svc::calc) fn unreg_effect(&mut self, item_key: UadItemKey, effect: &ad::AEffectRt) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.remove_entry(&item_key, &effect.ae.id);
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

fn uses_default_attrs(effect: &ad::AEffectRt) -> bool {
    match &effect.ae.buff {
        Some(buff_info) => matches!(buff_info.source, ad::AEffectBuffSrc::DefaultAttrs),
        _ => false,
    }
}
