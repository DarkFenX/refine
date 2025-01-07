use crate::{
    ad,
    defs::{EAttrId, EEffectId, SolItemId},
    sol::svc::calc::SolRawModifier,
    util::StMapSetL1,
};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolBuffRegister {
    pub(super) effects: StMapSetL1<SolItemId, EEffectId>,
    pub(super) modifiers: StMapSetL1<(SolItemId, EAttrId), SolRawModifier>,
}
impl SolBuffRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            effects: StMapSetL1::new(),
            modifiers: StMapSetL1::new(),
        }
    }
    // Effect methods
    pub(in crate::sol::svc::calc) fn get_effects(
        &self,
        item_id: &SolItemId,
    ) -> impl ExactSizeIterator<Item = &EEffectId> {
        self.effects.get(&item_id)
    }
    pub(in crate::sol::svc::calc) fn reg_effect(&mut self, item_id: SolItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.add_entry(item_id, effect.id);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_effect(&mut self, item_id: SolItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.remove_entry(&item_id, &effect.id);
        }
    }
    // Modifier methods
    pub(in crate::sol::svc::calc) fn extract_dependent_mods(
        &mut self,
        item_id: &SolItemId,
        buff_type_attr_id: &EAttrId,
    ) -> Option<impl ExactSizeIterator<Item = SolRawModifier>> {
        self.modifiers.remove_key(&(*item_id, *buff_type_attr_id))
    }
    pub(in crate::sol::svc::calc) fn reg_dependent_mod(
        &mut self,
        item_id: SolItemId,
        buff_type_attr_id: EAttrId,
        modifier: SolRawModifier,
    ) {
        self.modifiers.add_entry((item_id, buff_type_attr_id), modifier)
    }
    pub(in crate::sol::svc::calc) fn unreg_dependent_mod(
        &mut self,
        item_id: &SolItemId,
        buff_type_attr_id: &EAttrId,
        modifier: &SolRawModifier,
    ) {
        self.modifiers.remove_entry(&(*item_id, *buff_type_attr_id), &modifier)
    }
}

fn uses_default_attrs(effect: &ad::AEffect) -> bool {
    match &effect.buff {
        Some(buff_info) => match buff_info.source {
            ad::AEffectBuffSrc::DefaultAttrs => true,
            _ => false,
        },
        _ => false,
    }
}
