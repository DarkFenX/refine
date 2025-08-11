use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeLoc, NEffectDmgKind, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_unrestricted_s2s},
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::TGT_ATTACK;
const A_EFFECT_ID: AEffectId = ac::effects::TGT_ATTACK;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Turret),
            charge: Some(NEffectCharge {
                // Autocharge attribute ID is defined just for completeness of data. CCP Kestrel
                // confirmed civilian guns use on-gun damage attributes, and ammo is possibly loaded
                // just for various side effects (e.g. ammo affecting module attributes, or shot
                // graphics). The library doesn't implement on-module autocharges just for this
                // effect.
                location: NEffectChargeLoc::TargetAttack(ac::attrs::AMMO_LOADED),
                activates_charge: false,
            }),
            proj_mult_getter: Some(get_proj_mult_normal_unrestricted_s2s),
            normal_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_r_effect: &REffect,
    _spool: Option<Spool>,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let projector_u_item = ctx.u_data.items.get(projector_key);
    let dmg_item_key = match projector_u_item.get_axt().unwrap().capacity > OF(0.0) {
        // If item has capacity but no charge - it is not dealing damage
        true => projector_u_item.get_charge_key()?,
        false => projector_key,
    };
    let dmg_mult = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DMG_MULT)?;
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, dmg_item_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, dmg_item_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, dmg_item_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, dmg_item_key, &ac::attrs::EXPL_DMG)?;
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
