use crate::{
    def::AttrVal,
    misc::EffectSpec,
    nd::{NEffectLocalOpcSpec, NEffectProjOpcSpec},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, eff_funcs, output::Output},
    ud::UItemKey,
};

// TODO: expensive operations from here to level above - proj data, resists, limit calculation

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
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            output *= charge_mult;
        }
        if let Some(ilimit_getter) = self.instance_limit
            && let Some(ilimit) = ilimit_getter(ctx, calc, item_key)
        {
            output.limit_amount(ilimit);
        }
        Some(output.get_total())
    }
}

impl NEffectProjOpcSpec<AttrVal> {
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
            if let Some(ilimit_getter) = self.instance_limit
                && let Some(ilimit) = ilimit_getter(ctx, calc, projectee_key)
            {
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
