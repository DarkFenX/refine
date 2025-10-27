use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId},
    def::{AttrVal, Count, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            dd_mods::update_effect,
            proj_mult::{get_dd_lance_proj_mult, get_noapp_simple_c2s_proj_mult, get_simple_mod_proj_attrs},
        },
    },
    rd,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::{UItem, UItemKey},
    util::floor_unerr,
};
// TODO: test if it uses center-to-surface range (might use surface-to-surface), and check if damage
// TODO: radius is needed to be added to range or not. Range is used in modifier application and
// TODO: damage application

const E_EFFECT_ID: EEffectId = ec::effects::DEBUFF_LANCE;
const A_EFFECT_ID: AEffectId = ac::effects::DEBUFF_LANCE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::Customized(vec![
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::REMOTE_REPAIR_IMPEDANCE, OF(-50.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::WARP_PENALTY, OF(100.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_DOCK_JUMP, OF(1.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_TETHER, OF(1.0)),
            ]),
            scope: AEffectBuffScope::Everything,
        }),
        adg_update_effect_fn: Some(|a_effect| update_effect(A_EFFECT_ID, a_effect)),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_simple_c2s_proj_mult),
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &rd::REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let mut dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let mut dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let mut dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let mut dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
    let delay_s =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_WARNING_DURATION)? / OF(1000.0);
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        let mult = get_dd_lance_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        dmg_em *= mult;
        dmg_therm *= mult;
        dmg_kin *= mult;
        dmg_expl *= mult;
    }
    Some(
        match calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DAMAGE_CYCLE_TIME)? {
            interval_ms if interval_ms > OF(0.0) => {
                let duration_s =
                    calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DAMAGE_DURATION)?
                        / OF(1000.0);
                let repeats = floor_unerr(duration_s / (interval_ms / OF(1000.0))).into_inner() as Count;
                Output::Complex(OutputComplex {
                    amount: DmgKinds {
                        em: dmg_em,
                        thermal: dmg_therm,
                        kinetic: dmg_kin,
                        explosive: dmg_expl,
                    },
                    delay: delay_s,
                    repeats,
                    interval: interval_ms,
                })
            }
            _ => Output::Simple(OutputSimple {
                amount: DmgKinds {
                    em: dmg_em,
                    thermal: dmg_therm,
                    kinetic: dmg_kin,
                    explosive: dmg_expl,
                },
                delay: delay_s,
            }),
        },
    )
}
