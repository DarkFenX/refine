use super::invar_data::{EffectLocalInvarData, EffectProjInvarData};
use crate::{
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrKey, REffect, REffectLocalOpcSpec, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, eff_funcs, output::Output},
    ud::UItemKey,
};

impl REffectLocalOpcSpec<AttrVal> {
    pub(in crate::svc::vast::vaste_stats) fn make_invar_data(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> EffectLocalInvarData {
        EffectLocalInvarData {
            ilimit: get_self_ilimit(ctx, calc, item_key, self.ilimit_attr_key),
        }
    }
    pub(in crate::svc::vast::vaste_stats) fn get_total(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        effect: &REffect,
        chargedness: Option<AttrVal>,
        invar_data: EffectLocalInvarData,
    ) -> Option<AttrVal> {
        let mut output = (self.base)(ctx, calc, item_key, effect)?;
        if let Some(charge_mult_getter) = self.charge_mult
            && let Some(chargedness) = chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            output *= charge_mult;
        }
        if let Some(ilimit) = invar_data.ilimit {
            output.limit_amount(ilimit);
        }
        Some(output.get_total())
    }
}

impl REffectProjOpcSpec<AttrVal> {
    pub(in crate::svc::vast::vaste_stats) fn make_invar_data(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        projector_effect: &REffect,
        projectee_key: Option<UItemKey>,
    ) -> EffectProjInvarData {
        let projectee_key = match projectee_key {
            Some(projectee_key) => projectee_key,
            None => {
                return EffectProjInvarData {
                    mult_pre: None,
                    ilimit: None,
                    mult_post: None,
                };
            }
        };
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        let mut mult_pre = OF(1.0);
        if let Some(proj_mult_getter) = self.proj_mult_pre {
            mult_pre *= proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        }
        match self.resist {
            Some(REffectResist::Standard)
                if let Some(resist_mult) =
                    eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key) =>
            {
                mult_pre *= resist_mult;
            }
            Some(REffectResist::Attr(resist_attr_key))
                if let Some(resist_mult) = eff_funcs::get_resist_mult_by_projectee_aspec(
                    ctx,
                    calc,
                    &AttrSpec::new(projectee_key, resist_attr_key),
                ) =>
            {
                mult_pre *= resist_mult;
            }
            _ => (),
        }
        let mult_post = match self.proj_mult_post {
            Some(proj_mult_getter) => {
                let mult = proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
                process_mult(mult)
            }
            None => None,
        };
        let ilimit = get_proj_ilimit(ctx, calc, projectee_key, self.ilimit_attr_key);
        EffectProjInvarData {
            mult_pre: process_mult(mult_pre),
            ilimit,
            mult_post,
        }
    }
    pub(in crate::svc::vast::vaste_stats) fn get_output(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        projector_effect: &REffect,
        chargedness: Option<AttrVal>,
        spool_mult: Option<AttrVal>,
        invar_data: EffectProjInvarData,
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
        // Pre-limit projection & resistance effect reduction
        if let Some(invar_mult) = invar_data.mult_pre {
            output *= invar_mult;
        }
        // Instance limit
        if let Some(ilimit) = invar_data.ilimit {
            output.limit_amount(ilimit);
        }
        // Post-limit projection effect reduction
        if let Some(invar_mult) = invar_data.mult_post {
            output *= invar_mult;
        }
        Some(output)
    }
    pub(in crate::svc::vast::vaste_stats) fn get_total(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        projector_effect: &REffect,
        chargedness: Option<AttrVal>,
        spool_mult: Option<AttrVal>,
        invar_data: EffectProjInvarData,
    ) -> Option<AttrVal> {
        let output = self.get_output(
            ctx,
            calc,
            projector_key,
            projector_effect,
            chargedness,
            spool_mult,
            invar_data,
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

fn process_mult(mult: AttrVal) -> Option<AttrVal> {
    match mult {
        OF(1.0) => None,
        v => Some(v),
    }
}
