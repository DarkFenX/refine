// In EVE, missiles are launched from center of an attacking ship. Ships have non-zero radius, and
// overview distance is calculated from surface to surface. To make missile range roughly match to
// their theoretical range, CCP added hidden flight time bonus, which depends on radius of the
// attacking ship. This effect implements it.

use smallvec::{SmallVec, smallvec};

use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectId, AItem, AItemEffectData, AItemId, AState},
    def::{AttrVal, OF},
    misc::EffectSpec,
    nd::{NEffect, NEffectHc, effect::data::shared::util::get_item_fit_ship_key},
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, CustomAffectorValueKind,
            Location, ModifierKind, Op, RawModifier,
        },
    },
    ud::{UItem, UItemKey},
    util::RMap,
};

const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_FLIGHT_TIME;
const SHIP_RADIUS: AAttrId = ac::attrs::RADIUS;
const MISSILE_VELOCITY: AAttrId = ac::attrs::MAX_VELOCITY;
const MISSILE_FLIGHT_TIME: AAttrId = ac::attrs::EXPLOSION_DELAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        hc: NEffectHc {
            calc_customizer: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

// ADG customizations
fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: AState::Disabled,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for item in a_items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DOT_MISSILE_LAUNCHING)
    }) {
        item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}

// Calc customizations
fn calc_add_custom_modifier(rmods: &mut Vec<RawModifier>, espec: EffectSpec) {
    let rmod = RawModifier {
        kind: ModifierKind::Local,
        affector_espec: espec,
        affector_value: AffectorValue::Custom(CustomAffectorValue {
            kind: CustomAffectorValueKind::MissileFlightTime,
            affector_attr_id: Some(MISSILE_VELOCITY),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            item_add_reviser: Some(revise_on_item_add_removal),
            item_remove_reviser: Some(revise_on_item_add_removal),
        }),
        op: Op::ExtraAdd,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_attr_id: MISSILE_FLIGHT_TIME,
        ..
    };
    rmods.push(rmod);
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let ship_key = get_item_fit_ship_key(ctx, espec.item_key)?;
    let missile_velocity = calc
        .get_item_attr_val_full(ctx, espec.item_key, &MISSILE_VELOCITY)
        .ok()?;
    let ship_radius = ctx.u_data.items.get(ship_key).get_direct_radius();
    // Missile flight time is stored in milliseconds, thus have to multiply by 1000
    let val = ship_radius / missile_velocity.dogma * OF(1000.0);
    if val.is_infinite() {
        return None;
    }
    // No need to register dependencies here, because missile velocity attribute is getting cleared
    // the regular modifier way, and ship radius is taken unmodified intentionally. Since it is
    // taken unmodified, it should stay as-is, and applied modification doesn't need to be cleared
    // up whenever modified value changes
    Some(val)
}

fn get_affector_info(ctx: SvcCtx, item_key: UItemKey) -> SmallVec<AffectorInfo, 1> {
    match get_item_fit_ship_key(ctx, item_key) {
        Some(ship_key) => {
            smallvec![
                AffectorInfo {
                    item_id: ctx.u_data.items.id_by_key(item_key),
                    attr_id: Some(MISSILE_VELOCITY),
                },
                // There is no dependency on modified ship radius, but we add it for informational
                // purposes nevertheless
                AffectorInfo {
                    item_id: ctx.u_data.items.id_by_key(ship_key),
                    attr_id: Some(SHIP_RADIUS),
                }
            ]
        }
        None => SmallVec::new(),
    }
}

fn revise_on_item_add_removal(
    ctx: SvcCtx,
    affector_key: UItemKey,
    _changed_key: UItemKey,
    changed_item: &UItem,
) -> bool {
    match changed_item {
        UItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_key()) == ctx.u_data.items.get(affector_key).get_fit_key()
        }
        _ => false,
    }
}
