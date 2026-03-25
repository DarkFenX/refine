use crate::{
    misc::Ecm,
    nd::NOutputGetter,
    num::{PValue, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

pub(crate) enum NEcmOutputGetter {
    Direct,
    Burst,
    Aoe,
    Bomb,
    Entity,
}
impl NOutputGetter for NEcmOutputGetter {
    type Instance = Ecm;
    type Xargs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        _xargs: Self::Xargs,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_direct(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, effect: &REffect) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, effect.duration_attr_rid, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: PValue::ZERO,
    }))
}

fn get_burst(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    Some(Output::Simple(OutputSimple {
        instance: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}

fn get_aoe(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_aoe_duration, Value::ZERO)? / Value::THOUSAND,
    );
    let delay = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay,
    }))
}

fn get_bomb(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    // Do not return ECM stats for non-ecm bombs
    if radar <= PValue::ZERO && magnetometric <= PValue::ZERO && gravimetric <= PValue::ZERO && ladar <= PValue::ZERO {
        return None;
    }
    Some(Output::Simple(OutputSimple {
        instance: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}

fn get_entity(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().ecm_jam_duration, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        instance: Ecm {
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
