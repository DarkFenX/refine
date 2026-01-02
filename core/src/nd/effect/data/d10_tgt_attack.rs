use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::DmgKinds,
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

const E_EFFECT_ID: EEffectId = ec::effects::TGT_ATTACK;
const A_EFFECT_ID: AEffectId = ac::effects::TGT_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            // Autocharge attribute ID is defined just for completeness of data. CCP Kestrel
            // confirmed civilian guns use on-gun damage attributes, and ammo is possibly loaded
            // just for various side effects (e.g. ammo affecting module attributes, or shot
            // graphics). The library doesn't implement on-module autocharges just for this
            // effect.
            location: NEffectChargeLoc::TargetAttack(ac::attrs::AMMO_LOADED),
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
    item_key: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let item = ctx.u_data.items.get(item_key);
    let dmg_item_key = match item.get_axt().unwrap().capacity > OF(0.0) {
        // If item has capacity but no charge - it is not dealing damage
        true => item.get_charge_key()?,
        false => item_key,
    };
    let dmg_mult = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().dmg_mult, OF(0.0))?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, ctx.ac().em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, ctx.ac().therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, ctx.ac().kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, ctx.ac().expl_dmg, OF(0.0))?;
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
