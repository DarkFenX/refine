use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::Spool,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        effect::data::shared::{
            base_opc::{get_ancillary_armor_mult_old, get_outgoing_rep_opc},
            proj_mult::get_full_noapp_proj_mult,
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        outgoing_armor_rep_opc_getter: Some(internal_get_outgoing_rep_opc),
        ..
    }
}

fn internal_get_outgoing_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    chargedness: Option<AttrVal>,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    let extra_mult = get_ancillary_armor_mult_old(ctx, calc, projector_key, chargedness);
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        None,
        projectee_key,
        get_full_noapp_proj_mult,
        ctx.ac().armor_dmg_amount,
        ctx.ac().armor_hp,
        extra_mult,
        false,
    )
}
