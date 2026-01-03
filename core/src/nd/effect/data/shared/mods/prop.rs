use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AEffectAffecteeFilter, AEffectLocation, AEffectModifier, AOp},
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    nd::effect::data::shared::util::get_item_fit_ship_key,
    rd::RAttrConsts,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AffectorValue, AggrMode, Calc, CalcOp, CustomAffectorValue,
            CustomAffectorValueKind, Location, ModifierKind, RawModifier,
        },
    },
    ud::UItemId,
};

// ADG customizations
pub(in crate::nd::effect::data) fn mk_prop_mass_mod() -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: ac::attrs::MASS_ADDITION,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::MASS,
    }
}
pub(in crate::nd::effect::data) fn mk_mwd_sig_mod() -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    }
}
pub(in crate::nd::effect::data) fn mk_mjd_sig_mod() -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS_PERCENT,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    }
}

// Calc customizations
pub(in crate::nd::effect::data) fn add_prop_speed_mod(
    rmods: &mut Vec<RawModifier>,
    attr_consts: &RAttrConsts,
    affector_espec: EffectSpec,
) {
    if let Some(max_velocity_rid) = attr_consts.max_velocity
        && let Some(speed_factor_rid) = attr_consts.speed_factor
        && attr_consts.speed_boost_factor.is_some()
        && attr_consts.mass.is_some()
    {
        let rmod = RawModifier {
            kind: ModifierKind::Local,
            affector_espec,
            affector_value: AffectorValue::Custom(CustomAffectorValue {
                kind: CustomAffectorValueKind::PropSpeedBoost,
                // Exposing just 1 on-item attribute here which should change more than the other
                // one, not to handle it via dependencies
                affector_attr_key: Some(speed_factor_rid),
                affector_info_getter: get_affector_info,
                mod_val_getter: get_mod_val,
                ..
            }),
            op: CalcOp::PostMul,
            aggr_mode: AggrMode::Stack,
            affectee_filter: AffecteeFilter::Direct(Location::Ship),
            affectee_attr_key: max_velocity_rid,
            ..
        };
        rmods.push(rmod);
    }
}

fn get_affector_info(ctx: SvcCtx, item_uid: UItemId) -> SmallVec<Affector, 1> {
    let mut info = SmallVec::new();
    if let Some(ship_uid) = get_item_fit_ship_key(ctx, item_uid)
        && let Some(speed_factor_rid) = ctx.ac().speed_factor
        && let Some(speed_boost_factor_rid) = ctx.ac().speed_boost_factor
        && let Some(mass_rid) = ctx.ac().mass
    {
        let item_id = ctx.u_data.items.eid_by_iid(item_uid);
        info.extend([
            Affector {
                item_id,
                attr_id: Some(ctx.u_data.src.get_attr(speed_factor_rid).a_id.into()),
            },
            Affector {
                item_id,
                attr_id: Some(ctx.u_data.src.get_attr(speed_boost_factor_rid).a_id.into()),
            },
            Affector {
                item_id: ctx.u_data.items.eid_by_iid(ship_uid),
                attr_id: Some(ctx.u_data.src.get_attr(mass_rid).a_id.into()),
            },
        ]);
    }
    info
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let ship_uid = get_item_fit_ship_key(ctx, espec.item_uid)?;
    let speed_boost = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().speed_factor)?;
    let thrust = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().speed_boost_factor)?;
    let mass = calc.get_item_oattr_odogma(ctx, ship_uid, ctx.ac().mass)?;
    let perc = speed_boost * thrust / mass;
    if !perc.is_finite() {
        return None;
    }
    let val = OF(1.0) + perc / OF(100.0);
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ctx.ac(), ship_uid, espec);
    Some(val)
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
