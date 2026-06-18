use smallvec::SmallVec;

use super::CalcCustomModStrength;
use crate::{
    api::AttrId,
    misc::{AttrSpec, EffectSpec},
    num::Value,
    rd::RAttrConsts,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AggrMode, Calc, CalcCustomModifier, CalcOp, Location, ModifierKind, RawModifier,
            modifier::ModStrength,
        },
    },
    ud::UItemId,
};

pub(super) fn make_rmod(attr_consts: &RAttrConsts, espec: EffectSpec) -> Option<RawModifier> {
    if attr_consts.speed_boost_factor.is_none() || attr_consts.mass.is_none() {
        return None;
    }
    Some(RawModifier {
        kind: ModifierKind::Local,
        affector_espec: espec,
        strength: ModStrength::Custom(CalcCustomModStrength {
            kind: CalcCustomModifier::PropSpeed,
            // Exposing just 1 on-item attribute here which should change more than the other
            // one, not to handle it via dependencies
            affector_attr_rid: Some(attr_consts.speed_factor?),
            ..
        }),
        op: CalcOp::PostMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Ship),
        affectee_attr_rid: attr_consts.max_velocity?,
        ..
    })
}

pub(super) fn get_affector_info(ctx: SvcCtx, item_uid: UItemId) -> SmallVec<[Affector; 1]> {
    let mut info = SmallVec::new();
    if let Some(ship_uid) = ctx.u_data.get_item_fit_ship_uid(item_uid)
        && let Some(speed_factor_rid) = ctx.ac().speed_factor
        && let Some(speed_boost_factor_rid) = ctx.ac().speed_boost_factor
        && let Some(mass_rid) = ctx.ac().mass
    {
        let item_id = ctx.u_data.items.ext_id_by_int_id(item_uid);
        info.extend([
            Affector {
                item_id,
                attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(speed_factor_rid).aid)),
            },
            Affector {
                item_id,
                attr_id: Some(AttrId::from_aid(
                    ctx.u_data.src.get_attr_by_rid(speed_boost_factor_rid).aid,
                )),
            },
            Affector {
                item_id: ctx.u_data.items.ext_id_by_int_id(ship_uid),
                attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(mass_rid).aid)),
            },
        ]);
    }
    info
}

pub(super) fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
    let ship_uid = ctx.u_data.get_item_fit_ship_uid(espec.item_uid)?;
    let speed_boost = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().speed_factor)?;
    let thrust = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().speed_boost_factor)?;
    let mass = calc.get_item_oattr_odogma(ctx, ship_uid, ctx.ac().mass)?;
    let perc = speed_boost * thrust / mass;
    if !perc.is_finite() {
        return None;
    }
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ctx.ac(), ship_uid, espec);
    Some(perc.perc_change_to_mult())
}

fn reg_dependencies(calc: &mut Calc, attr_consts: &RAttrConsts, ship_uid: UItemId, prop_espec: EffectSpec) {
    // Prop boost attribute is declared the usual way, everything else is declared here
    if let Some(speed_boost_factor_rid) = attr_consts.speed_boost_factor
        && let Some(mass_rid) = attr_consts.mass
        && let Some(max_velocity_rid) = attr_consts.max_velocity
    {
        let affectee_aspec = AttrSpec::new(ship_uid, max_velocity_rid);
        calc.deps.add_with_source(
            prop_espec,
            AttrSpec::new(prop_espec.item_uid, speed_boost_factor_rid),
            affectee_aspec,
        );
        calc.deps
            .add_with_source(prop_espec, AttrSpec::new(ship_uid, mass_rid), affectee_aspec);
    }
}
