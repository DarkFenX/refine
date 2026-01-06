use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, PValue},
    nd::{
        NEffect, NEffectCharge, NEffectChargeLoc, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::proj_mult::get_turret_proj_mult,
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemId},
};

const EFFECT_EID: EEffectId = EEffectId::TGT_ATTACK;
const EFFECT_AID: AEffectId = AEffectId::TGT_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            // Autocharge attribute ID is defined just for completeness of data. CCP Kestrel
            // confirmed civilian guns use on-gun damage attributes, and ammo is possibly loaded
            // just for various side effects (e.g. ammo affecting module attributes, or shot
            // graphics). The library doesn't implement on-module autocharges just for this
            // effect.
            location: NEffectChargeLoc::TargetAttack(AAttrId::AMMO_LOADED),
            activates_charge: false,
        }),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_dmg_base_opc,
            proj_mult_str: Some(get_turret_proj_mult),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Turret
}

fn internal_get_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let item = ctx.u_data.items.get(item_uid);
    let dmg_dealer_uid = match item.get_axt().unwrap().capacity > PValue::from_f64_unchecked(0.0) {
        // If item has capacity but no charge - it is not dealing damage
        true => item.get_charge_uid()?,
        false => item_uid,
    };
    let dmg_mult = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_mult, OF(0.0))?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, dmg_dealer_uid, ctx.ac().em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, dmg_dealer_uid, ctx.ac().therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, dmg_dealer_uid, ctx.ac().kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, dmg_dealer_uid, ctx.ac().expl_dmg, OF(0.0))?;
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em * dmg_mult,
            thermal: dmg_therm * dmg_mult,
            kinetic: dmg_kin * dmg_mult,
            explosive: dmg_expl * dmg_mult,
        },
        delay: OF(0.0),
    }))
}
