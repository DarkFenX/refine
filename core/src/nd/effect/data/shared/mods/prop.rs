use smallvec::{SmallVec, smallvec};

use crate::{
    ac,
    ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModifier, AOp},
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    nd::effect::data::shared::util::get_item_fit_ship_key,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, CustomAffectorValueKind,
            Location, ModifierKind, Op, RawModifier,
        },
    },
    ud::UItemKey,
};

const SHIP_MASS: AAttrId = ac::attrs::MASS;
const SHIP_SPEED: AAttrId = ac::attrs::MAX_VELOCITY;
const PROP_THRUST: AAttrId = ac::attrs::SPEED_BOOST_FACTOR;
const PROP_BOOST: AAttrId = ac::attrs::SPEED_FACTOR;

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
pub(in crate::nd::effect::data) fn add_prop_speed_mod(rmods: &mut Vec<RawModifier>, affector_espec: EffectSpec) {
    let rmod = RawModifier {
        kind: ModifierKind::Local,
        affector_espec,
        affector_value: AffectorValue::Custom(CustomAffectorValue {
            kind: CustomAffectorValueKind::PropSpeedBoost,
            // Exposing just 1 on-item attribute here which should change more than the other one,
            // not to handle it via dependencies
            affector_attr_id: Some(PROP_BOOST),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            ..
        }),
        op: Op::PostMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Ship),
        affectee_attr_id: SHIP_SPEED,
        ..
    };
    rmods.push(rmod);
}

fn get_affector_info(ctx: SvcCtx, item_key: UItemKey) -> SmallVec<AffectorInfo, 1> {
    match get_item_fit_ship_key(ctx, item_key) {
        Some(ship_key) => {
            let item_id = ctx.u_data.items.id_by_key(item_key);
            smallvec![
                AffectorInfo {
                    item_id,
                    attr_id: Some(PROP_BOOST),
                },
                AffectorInfo {
                    item_id,
                    attr_id: Some(PROP_THRUST),
                },
                AffectorInfo {
                    item_id: ctx.u_data.items.id_by_key(ship_key),
                    attr_id: Some(SHIP_MASS),
                }
            ]
        }
        None => SmallVec::new(),
    }
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let ship_key = get_item_fit_ship_key(ctx, espec.item_key)?;
    let speed_boost = calc.get_item_attr_val_full(ctx, espec.item_key, &PROP_BOOST).ok()?;
    let thrust = calc.get_item_attr_val_full(ctx, espec.item_key, &PROP_THRUST).ok()?;
    let mass = calc.get_item_attr_val_full(ctx, ship_key, &SHIP_MASS).ok()?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return None;
    }
    let val = OF(1.0) + perc / OF(100.0);
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_key, espec);
    Some(val)
}

fn reg_dependencies(calc: &mut Calc, ship_key: UItemKey, prop_espec: EffectSpec) {
    // Prop boost attribute is declared the usual way, everything else is declared here
    let affectee_aspec = AttrSpec::new(ship_key, SHIP_SPEED);
    calc.deps.add_with_source(
        prop_espec,
        AttrSpec::new(prop_espec.item_key, PROP_THRUST),
        affectee_aspec,
    );
    calc.deps
        .add_with_source(prop_espec, AttrSpec::new(ship_key, SHIP_MASS), affectee_aspec);
}
