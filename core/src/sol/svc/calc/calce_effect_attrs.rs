use crate::{
    ad,
    sol::{
        AttrVal, ItemKey,
        svc::{SvcCtx, calc::Calc},
    },
};

impl Calc {
    pub(in crate::sol::svc::calc) fn get_item_effect_id_duration(
        &mut self,
        ctx: &SvcCtx,
        item_key: ItemKey,
        a_effect_id: &ad::AEffectId,
    ) -> Option<AttrVal> {
        let a_effect = ctx.uad.src.get_a_effect(a_effect_id)?;
        self.get_item_effect_duration(ctx, item_key, a_effect)
    }
    pub(in crate::sol::svc::calc) fn get_item_effect_duration(
        &mut self,
        ctx: &SvcCtx,
        item_key: ItemKey,
        a_effect: &ad::ArcEffectRt,
    ) -> Option<AttrVal> {
        let attr_id = a_effect.ae.duration_attr_id?;
        let val = self.get_item_attr_val_full(ctx, item_key, &attr_id).ok()?;
        Some(val.dogma)
    }
}
