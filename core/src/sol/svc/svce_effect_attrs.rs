use crate::{
    ad,
    defs::{AttrVal, EEffectId, SolItemId},
    sol::{svc::SolSvc, uad::SolUad},
};

impl SolSvc {
    pub(in crate::sol::svc) fn get_item_effect_id_duration(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        effect_id: &EEffectId,
    ) -> Option<AttrVal> {
        let effect = uad.src.get_a_effect(effect_id)?;
        self.get_item_effect_duration(uad, item_id, effect)
    }
    pub(in crate::sol::svc) fn get_item_effect_duration(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        effect: &ad::ArcEffect,
    ) -> Option<AttrVal> {
        let attr_id = effect.duration_attr_id?;
        let val = self.calc_get_item_attr_val(uad, item_id, &attr_id).ok()?;
        Some(val.dogma)
    }
}
