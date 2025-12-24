use crate::{
    def::AttrVal,
    nd::NEffectLocalOpcSpec,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

impl NEffectLocalOpcSpec<AttrVal> {
    pub(super) fn get_total(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        effect: &REffect,
        chargedness: Option<AttrVal>,
    ) -> Option<AttrVal> {
        let mut output = (self.base)(ctx, calc, item_key, effect)?;
        if let Some(charge_mult_getter) = self.charge_mult
            && let Some(chargedness) = chargedness
            && let Some(mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            output *= mult;
        }
        if let Some(ilimit_getter) = self.instance_limit
            && let Some(ilimit) = ilimit_getter(ctx, calc, item_key)
        {
            output.limit_amount(ilimit);
        }
        Some(output.get_total())
    }
}
