use smallvec::{SmallVec, smallvec};

use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    nd::{NEffect, NEffectHc, eff::shared::util::get_item_fit_ship_key},
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, CustomAffectorValueKind,
            Location, ModifierKind, Op, RawModifier,
        },
    },
    uad::UadItem,
    util::RMap,
};

const A_EFFECT_ID: ad::AEffectId = ac::effects::MISSILE_FLIGHT_TIME;
const SHIP_RADIUS: ad::AAttrId = ac::attrs::RADIUS;
const MISSILE_VELOCITY: ad::AAttrId = ac::attrs::MAX_VELOCITY;
const MISSILE_FLIGHT_TIME: ad::AAttrId = ac::attrs::EXPLOSION_DELAY;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        hc: NEffectHc {
            calc_custom_fn: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

// ADG customizations
fn make_effect() -> ad::AEffect {
    ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<ad::AItemId, ad::AItem>) -> bool {
    let mut assigned = false;
    for item in a_items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DOT_MISSILE_LAUNCHING)
    }) {
        item.effect_datas.insert(A_EFFECT_ID, ad::AItemEffectData::default());
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
            affector_a_attr_id: Some(MISSILE_VELOCITY),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            item_add_reviser: Some(revise_on_item_add_removal),
            item_remove_reviser: Some(revise_on_item_add_removal),
        }),
        op: Op::ExtraAdd,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_a_attr_id: MISSILE_FLIGHT_TIME,
        ..
    };
    rmods.push(rmod);
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let ship_key = get_item_fit_ship_key(ctx, espec.item_key)?;
    let missile_velocity = calc
        .get_item_attr_val_full(ctx, espec.item_key, &MISSILE_VELOCITY)
        .ok()?;
    let ship_radius = ctx.uad.get_item_radius(ship_key);
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

fn get_affector_info(ctx: SvcCtx, item_key: ItemKey) -> SmallVec<AffectorInfo, 1> {
    match get_item_fit_ship_key(ctx, item_key) {
        Some(ship_key) => {
            smallvec![
                AffectorInfo {
                    item_id: ctx.uad.items.id_by_key(item_key),
                    attr_id: Some(MISSILE_VELOCITY),
                },
                // There is no dependency on modified ship radius, but we add it for informational
                // purposes nevertheless
                AffectorInfo {
                    item_id: ctx.uad.items.id_by_key(ship_key),
                    attr_id: Some(SHIP_RADIUS),
                }
            ]
        }
        None => SmallVec::new(),
    }
}

fn revise_on_item_add_removal(
    ctx: SvcCtx,
    affector_key: ItemKey,
    _changed_key: ItemKey,
    changed_item: &UadItem,
) -> bool {
    match changed_item {
        UadItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_key()) == ctx.uad.items.get(affector_key).get_fit_key()
        }
        _ => false,
    }
}
