use smallvec::{SmallVec, smallvec};

use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::{AttrSpec, EffectSpec},
    nd::eff::shared::util::get_item_fit_ship_key,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, CustomAffectorValueKind,
            Location, ModifierKind, Op, RawModifier,
        },
    },
};

const SHIP_MASS: ad::AAttrId = ac::attrs::MASS;
const SHIP_SPEED: ad::AAttrId = ac::attrs::MAX_VELOCITY;
const PROP_THRUST: ad::AAttrId = ac::attrs::SPEED_BOOST_FACTOR;
const PROP_BOOST: ad::AAttrId = ac::attrs::SPEED_FACTOR;

// ADG customizations
pub(in crate::nd::eff) fn mk_a_modifier_mass() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::MASS_ADDITION,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::MASS,
    }
}

pub(in crate::nd::eff) fn mk_a_modifier_sig() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    }
}

// Calc customizations
pub(in crate::nd::eff) fn calc_add_custom_modifier(rmods: &mut Vec<RawModifier>, affector_espec: EffectSpec) {
    let rmod = RawModifier {
        kind: ModifierKind::Local,
        affector_espec,
        affector_value: AffectorValue::Custom(CustomAffectorValue {
            kind: CustomAffectorValueKind::PropSpeedBoost,
            // Exposing just 1 on-item attribute here which should change more than the other one,
            // not to handle it via dependencies
            affector_a_attr_id: Some(PROP_BOOST),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            ..
        }),
        op: Op::PostMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Ship),
        affectee_a_attr_id: SHIP_SPEED,
        ..
    };
    rmods.push(rmod);
}

fn get_affector_info(ctx: SvcCtx, item_key: ItemKey) -> SmallVec<AffectorInfo, 1> {
    match get_item_fit_ship_key(ctx, item_key) {
        Some(ship_key) => {
            let item_id = ctx.uad.items.id_by_key(item_key);
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
                    item_id: ctx.uad.items.id_by_key(ship_key),
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

fn reg_dependencies(calc: &mut Calc, ship_key: ItemKey, prop_espec: EffectSpec) {
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
