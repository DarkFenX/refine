use crate::{
    nd::NEffectOutputGetter,
    num::{PValue, Value},
    rd::{RAttrId, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

// To avoid excessive monomorphization, all the getters which produce PValue are aggregated into
// single enum
#[derive(Copy, Clone)]
pub(crate) enum NEffectGeneralOutputGetter {
    CapConsumer,
    RepShield,
    RepArmor,
    RepHull,
    PowerTransfer,
    Neut,
    NeutNosf,
    NeutAoe,
    NeutBomb,
    NeutDdWarmup,
    // Variants specific to a single effect
    PowerBooster,
}
impl NEffectOutputGetter for NEffectGeneralOutputGetter {
    type Instance = PValue;
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
            Self::CapConsumer => get_attr(ctx, calc, item_uid, effect, effect.discharge_attr_rid, true),
            Self::RepShield => get_attr(ctx, calc, item_uid, effect, ctx.ac().shield_bonus, true),
            Self::RepArmor => get_attr(ctx, calc, item_uid, effect, ctx.ac().armor_dmg_amount, false),
            Self::RepHull => get_attr(ctx, calc, item_uid, effect, ctx.ac().struct_dmg_amount, false),
            Self::PowerTransfer => get_attr(ctx, calc, item_uid, effect, ctx.ac().power_transfer_amount, false),
            Self::Neut => get_attr(ctx, calc, item_uid, effect, ctx.ac().energy_neut_amount, true),
            Self::NeutNosf => get_neut_nosf(ctx, calc, item_uid, effect),
            Self::NeutAoe => get_neut_aoe(ctx, calc, item_uid),
            Self::NeutBomb => get_neut_bomb(ctx, calc, item_uid),
            Self::NeutDdWarmup => get_attr(ctx, calc, item_uid, effect, ctx.ac().doomsday_energy_neut_amount, true),
            Self::PowerBooster => get_power_booster(ctx, calc, item_uid),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_attr(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    attr_rid: Option<RAttrId>,
    applied_at_start: bool,
) -> Option<Output<PValue>> {
    Some(Output::Simple(OutputSimple {
        instance: PValue::from_value_clamped(calc.get_item_oattr_afb_odogma(ctx, item_uid, attr_rid, Value::ZERO)?),
        delay: match applied_at_start {
            true => PValue::ZERO,
            false => funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?,
        },
    }))
}

fn get_neut_nosf(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, effect: &REffect) -> Option<Output<PValue>> {
    // Not a blood raider ship - not considered as a neut
    if calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().nos_override)?.abs() < PValue::FLOAT_TOLERANCE {
        return None;
    }
    get_attr(ctx, calc, item_uid, effect, ctx.ac().power_transfer_amount, false)
}

fn get_neut_aoe(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<PValue>> {
    let instance = PValue::from_value_clamped(calc.get_item_oattr_afb_odogma(
        ctx,
        item_uid,
        ctx.ac().energy_neut_amount,
        Value::ZERO,
    )?);
    let delay = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple { instance, delay }))
}

fn get_neut_bomb(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<PValue>> {
    let instance = calc.get_item_oattr_afb_odogma(ctx, item_uid, ctx.ac().energy_neut_amount, Value::ZERO)?;
    let instance = match instance > Value::ZERO {
        true => PValue::from_value_unchecked(instance),
        // Do not return neut output for non-neut bombs
        false => return None,
    };
    Some(Output::Simple(OutputSimple {
        instance,
        delay: PValue::ZERO,
    }))
}

fn get_power_booster(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<PValue>> {
    let item = ctx.u_data.items.get(item_uid);
    let charge_uid = item.get_charge_uid()?;
    let attr_consts = ctx.ac();
    let instance = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        charge_uid,
        attr_consts.capacitor_bonus,
        Value::ZERO,
    )?);
    Some(Output::Simple(OutputSimple {
        instance,
        delay: PValue::ZERO,
    }))
}
