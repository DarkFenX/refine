use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeLoc, NEffectDmgKind, effect::data::shared::proj_mult::get_turret_proj_mult,
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemKey},
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
        normal_dmg_opc_getter: Some(get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Turret
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let projector_u_item = ctx.u_data.items.get(projector_key);
    let dmg_item_key = match projector_u_item.get_axt().unwrap().capacity > OF(0.0) {
        // If item has capacity but no charge - it is not dealing damage
        true => projector_u_item.get_charge_key()?,
        false => projector_key,
    };
    let attr_consts = ctx.ac();
    let mut dmg_mult = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dmg_mult, OF(0.0))?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, attr_consts.em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, attr_consts.therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, attr_consts.kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, dmg_item_key, attr_consts.expl_dmg, OF(0.0))?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        dmg_mult *= get_turret_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    }
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
