use crate::{
    ad,
    sol::{
        ItemKey,
        svc::{AttrSpec, calc::RawModifier},
    },
    util::RMapRSet,
};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct BuffRegister {
    pub(super) a_effect_ids: RMapRSet<ItemKey, ad::AEffectId>,
    pub(super) modifiers: RMapRSet<AttrSpec, RawModifier>,
}
impl BuffRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            a_effect_ids: RMapRSet::new(),
            modifiers: RMapRSet::new(),
        }
    }
    // Effect methods
    pub(in crate::sol::svc::calc) fn get_effects(
        &self,
        item_key: &ItemKey,
    ) -> impl ExactSizeIterator<Item = &ad::AEffectId> {
        self.a_effect_ids.get(item_key)
    }
    pub(in crate::sol::svc::calc) fn reg_effect(&mut self, item_key: ItemKey, effect: &ad::AEffectRt) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.add_entry(item_key, effect.ae.id);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_effect(&mut self, item_key: ItemKey, effect: &ad::AEffectRt) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.remove_entry(&item_key, &effect.ae.id);
        }
    }
    // Modifier methods
    pub(in crate::sol::svc::calc) fn extract_dependent_mods(
        &mut self,
        aspec: &AttrSpec,
    ) -> Option<impl ExactSizeIterator<Item = RawModifier> + use<>> {
        self.modifiers.remove_key(aspec)
    }
    pub(in crate::sol::svc::calc) fn reg_dependent_mod(&mut self, aspec: AttrSpec, modifier: RawModifier) {
        self.modifiers.add_entry(aspec, modifier)
    }
    pub(in crate::sol::svc::calc) fn unreg_dependent_mod(&mut self, aspec: &AttrSpec, modifier: &RawModifier) {
        self.modifiers.remove_entry(aspec, modifier);
    }
}

fn uses_default_attrs(effect: &ad::AEffectRt) -> bool {
    match &effect.ae.buff {
        Some(buff_info) => matches!(buff_info.source, ad::AEffectBuffSrc::DefaultAttrs),
        _ => false,
    }
}
