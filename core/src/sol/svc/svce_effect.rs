use crate::{
    ad,
    defs::{AttrVal, EEffectId, SolItemId},
    sol::{svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(in crate::sol::svc) fn get_item_effect_id_duration(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        effect_id: &EEffectId,
    ) -> Option<AttrVal> {
        let effect = sol_view.src.get_a_effect(effect_id)?;
        self.get_item_effect_duration(sol_view, item_id, effect)
    }
    pub(in crate::sol::svc) fn get_item_effect_duration(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        effect: &ad::ArcEffect,
    ) -> Option<AttrVal> {
        let attr_id = effect.duration_attr_id?;
        let val = self.calc_get_item_attr_val(sol_view, item_id, &attr_id).ok()?;
        Some(val.dogma)
    }
}
