use crate::{
    nd::{NEffectOutputGetter, NEffectProjOpcSpec},
    num::{PValue, Value},
    rd::{RAttrConsts, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemId},
};

pub(crate) struct NEffectEcm {
    pub(crate) checker: Option<NEffectEcmChecker>,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectEcmOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Base item checker
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectEcmChecker {
    Bomb,
}
impl NEffectEcmChecker {
    pub(crate) fn check(&self, u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
        match self {
            Self::Bomb => check_bomb(u_item, attr_consts),
        }
    }
}

fn check_bomb(u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
    u_item.get_oattr_ffb(attr_consts.scan_radar_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_magnetometric_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_gravimetric_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_ladar_strength_bonus, Value::ZERO) > Value::ZERO
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output amount
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct NEffectEcmAmount {
    pub(crate) radar: PValue,
    pub(crate) magnetometric: PValue,
    pub(crate) gravimetric: PValue,
    pub(crate) ladar: PValue,
    pub(crate) duration: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectEcmOutputGetter {
    Direct,
    Burst,
    Aoe,
    Bomb,
    Entity,
}
impl NEffectOutputGetter for NEffectEcmOutputGetter {
    type Instance = NEffectEcmAmount;
    type XArgs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        _xargs: Self::XArgs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Direct => get_direct(ctx, calc, item_uid, effect),
            Self::Burst => get_burst(ctx, calc, item_uid),
            Self::Aoe => get_aoe(ctx, calc, item_uid),
            Self::Bomb => get_bomb(ctx, calc, item_uid),
            Self::Entity => get_entity(ctx, calc, item_uid),
        }
    }
}

fn get_direct(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, effect: &REffect) -> Option<Output<NEffectEcmAmount>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, effect.duration_attr_rid, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: NEffectEcmAmount {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: PValue::ZERO,
    }))
}

fn get_burst(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<NEffectEcmAmount>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    Some(Output::Simple(OutputSimple {
        instance: NEffectEcmAmount {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}

fn get_aoe(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<NEffectEcmAmount>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_aoe_duration, Value::ZERO)? / Value::THOUSAND,
    );
    let delay = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: NEffectEcmAmount {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay,
    }))
}

fn get_bomb(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<NEffectEcmAmount>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    // Do not return ECM stats for non-ecm bombs
    if radar <= PValue::ZERO && magnetometric <= PValue::ZERO && gravimetric <= PValue::ZERO && ladar <= PValue::ZERO {
        return None;
    }
    Some(Output::Simple(OutputSimple {
        instance: NEffectEcmAmount {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}

fn get_entity(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<NEffectEcmAmount>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().ecm_jam_duration, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: NEffectEcmAmount {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: PValue::ZERO,
    }))
}

fn get_ecm_values(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<(PValue, PValue, PValue, PValue)> {
    Some((
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_radar_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_magnetometric_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_gravimetric_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_ladar_strength_bonus,
            Value::ZERO,
        )?),
    ))
}
