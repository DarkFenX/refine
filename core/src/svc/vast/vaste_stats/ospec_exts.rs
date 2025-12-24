use crate::{
    def::AttrVal,
    misc::EffectSpec,
    rd::{RAttrKey, REffect, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{SvcCtx, calc::Calc, eff_funcs, output::Output},
    ud::UItemKey,
};

// TODO: expensive operations from here to level above - proj data, resists, limit calculation

impl REffectLocalOpcSpec<AttrVal> {
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
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            output *= charge_mult;
        }
        if let Some(ilimit) = get_self_ilimit(ctx, calc, item_key, self.ilimit_attr_key) {
            output.limit_amount(ilimit);
        }
        Some(output.get_total())
    }
}

impl REffectProjOpcSpec<AttrVal> {
    pub(super) fn get_output(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        projector_effect: &REffect,
        chargedness: Option<AttrVal>,
        spool_mult: Option<AttrVal>,
        projectee_key: Option<UItemKey>,
    ) -> Option<Output<AttrVal>> {
        let mut output = (self.base)(ctx, calc, projector_key, projector_effect)?;
        // Chargedness
        if let Some(charge_mult_getter) = self.charge_mult
            && let Some(chargedness) = chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
        {
            output *= charge_mult;
        }
        // Spool
        if let Some(spool_mult) = spool_mult {
            output *= spool_mult;
        }
        if let Some(projectee_key) = projectee_key {
            let proj_data = ctx.eff_projs.get_or_make_proj_data(
                ctx.u_data,
                EffectSpec::new(projector_key, projector_effect.key),
                projectee_key,
            );
            // Projection reduction
            output *= (self.proj_mult)(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
            // Effect resistance reduction
            if let Some(rr_mult) =
                eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
            {
                output *= rr_mult;
            }
            // Resource pool limit
            if let Some(ilimit) = get_proj_ilimit(ctx, calc, projectee_key, self.ilimit_attr_key) {
                output.limit_amount(ilimit);
            }
        }
        Some(output)
    }
    pub(super) fn get_total(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        projector_effect: &REffect,
        chargedness: Option<AttrVal>,
        spool_mult: Option<AttrVal>,
        projectee_key: Option<UItemKey>,
    ) -> Option<AttrVal> {
        let output = self.get_output(
            ctx,
            calc,
            projector_key,
            projector_effect,
            chargedness,
            spool_mult,
            projectee_key,
        )?;
        Some(output.get_total())
    }
}

fn get_self_ilimit(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, attr_key: Option<RAttrKey>) -> Option<AttrVal> {
    let attr_key = attr_key?;
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_attr_oextra(ctx, ship_key, attr_key)
}

fn get_proj_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_key: UItemKey,
    attr_key: Option<RAttrKey>,
) -> Option<AttrVal> {
    calc.get_item_oattr_oextra(ctx, projectee_key, attr_key)
}
