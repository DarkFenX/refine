use crate::{
    ad,
    sol::{AttrVal, ItemId, svc::calc::Calc, uad::Uad},
};

impl Calc {
    pub(in crate::sol::svc::calc) fn get_item_effect_id_duration(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_effect_id: &ad::AEffectId,
    ) -> Option<AttrVal> {
        let a_effect = uad.src.get_a_effect(a_effect_id)?;
        self.get_item_effect_duration(uad, item_id, a_effect)
    }
    pub(in crate::sol::svc::calc) fn get_item_effect_duration(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_effect: &ad::ArcEffect,
    ) -> Option<AttrVal> {
        let attr_id = a_effect.duration_attr_id?;
        let val = self.get_item_attr_val_full(uad, item_id, &attr_id).ok()?;
        Some(val.dogma)
    }
}
