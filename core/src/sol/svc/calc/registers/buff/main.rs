use crate::{
    ad,
    sol::{ItemId, svc::calc::RawModifier},
    util::StMapSetL1,
};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct BuffRegister {
    pub(super) a_effect_ids: StMapSetL1<ItemId, ad::AEffectId>,
    pub(super) modifiers: StMapSetL1<(ItemId, ad::AAttrId), RawModifier>,
}
impl BuffRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            a_effect_ids: StMapSetL1::new(),
            modifiers: StMapSetL1::new(),
        }
    }
    // Effect methods
    pub(in crate::sol::svc::calc) fn get_effects(
        &self,
        item_id: &ItemId,
    ) -> impl ExactSizeIterator<Item = &ad::AEffectId> {
        self.a_effect_ids.get(item_id)
    }
    pub(in crate::sol::svc::calc) fn reg_effect(&mut self, item_id: ItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.add_entry(item_id, effect.id);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_effect(&mut self, item_id: ItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.a_effect_ids.remove_entry(&item_id, &effect.id);
        }
    }
    // Modifier methods
    pub(in crate::sol::svc::calc) fn extract_dependent_mods(
        &mut self,
        item_id: &ItemId,
        buff_type_a_attr_id: &ad::AAttrId,
    ) -> Option<impl ExactSizeIterator<Item = RawModifier> + use<>> {
        self.modifiers.remove_key(&(*item_id, *buff_type_a_attr_id))
    }
    pub(in crate::sol::svc::calc) fn reg_dependent_mod(
        &mut self,
        item_id: ItemId,
        buff_type_a_attr_id: ad::AAttrId,
        modifier: RawModifier,
    ) {
        self.modifiers.add_entry((item_id, buff_type_a_attr_id), modifier)
    }
    pub(in crate::sol::svc::calc) fn unreg_dependent_mod(
        &mut self,
        item_id: &ItemId,
        buff_type_a_attr_id: &ad::AAttrId,
        modifier: &RawModifier,
    ) {
        self.modifiers.remove_entry(&(*item_id, *buff_type_a_attr_id), modifier);
    }
}

fn uses_default_attrs(effect: &ad::AEffect) -> bool {
    match &effect.buff {
        Some(buff_info) => matches!(buff_info.source, ad::AEffectBuffSrc::DefaultAttrs),
        _ => false,
    }
}
