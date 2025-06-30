use smallvec::{SmallVec, smallvec};

use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::{AttrSpec, EffectSpec},
    ntt::{NttEffect, NttEffectRt, eff::shared::util::get_item_fit_ship_key},
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, CustomAffectorValueKind,
            Location, ModifierKind, Op, RawModifier,
        },
    },
    uad::UadItem,
};

const A_EFFECT_ID: ad::AEffectId = ac::effects::MISSILE_FLIGHT_TIME;
const SHIP_RADIUS: ad::AAttrId = ac::attrs::RADIUS;
const MISSILE_VELOCITY: ad::AAttrId = ac::attrs::MAX_VELOCITY;
const MISSILE_FLIGHT_TIME: ad::AAttrId = ac::attrs::EXPLOSION_DELAY;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(add_custom_effect),
        rt: NttEffectRt {
            calc_custom_fn: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

// ADG customizations
fn add_custom_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        ..
    };
    let effect_id = effect.id;
    a_data.effects.insert(effect.id, effect);
    for item in a_data.items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DOT_MISSILE_LAUNCHING)
    }) {
        item.effect_datas.insert(effect_id, ad::AItemEffectData::default());
    }
}

// Calc customizations
fn calc_add_custom_modifier(rmods: &mut Vec<RawModifier>, espec: EffectSpec) {
    let rmod = RawModifier {
        kind: ModifierKind::Local,
        affector_espec: espec,
        affector_value: AffectorValue::Custom(CustomAffectorValue {
            kind: CustomAffectorValueKind::MissileFlightTime,
            affector_a_attr_id: None,
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

fn get_mod_val(calc: &mut Calc, ctx: &SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let ship_key = get_item_fit_ship_key(ctx, espec.item_key)?;
    let missile_velocity = calc
        .get_item_attr_val_full(ctx, espec.item_key, &MISSILE_VELOCITY)
        .ok()?;
    let ship_radius = calc.get_item_attr_val_full(ctx, ship_key, &SHIP_RADIUS).ok()?;
    // Missile flight time is stored in milliseconds, thus have to multiply by 1000
    let val = ship_radius.dogma / missile_velocity.dogma * OF(1000.0);
    if val.is_infinite() {
        return None;
    }
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_key, espec);
    Some(val)
}

fn get_affector_info(ctx: &SvcCtx, item_key: ItemKey) -> SmallVec<AffectorInfo, 1> {
    match get_item_fit_ship_key(ctx, item_key) {
        Some(ship_key) => {
            smallvec!(
                AffectorInfo {
                    item_id: ctx.uad.items.id_by_key(item_key),
                    attr_id: Some(MISSILE_VELOCITY),
                },
                AffectorInfo {
                    item_id: ctx.uad.items.id_by_key(ship_key),
                    attr_id: Some(SHIP_RADIUS),
                }
            )
        }
        None => SmallVec::new(),
    }
}

fn revise_on_item_add_removal(
    ctx: &SvcCtx,
    affector_key: ItemKey,
    _changed_item_key: ItemKey,
    changed_item: &UadItem,
) -> bool {
    match changed_item {
        UadItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_key()) == ctx.uad.items.get(affector_key).get_fit_key()
        }
        _ => false,
    }
}

fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, missile_espec: EffectSpec) {
    let affectee_aspec = AttrSpec::new(missile_espec.item_key, MISSILE_FLIGHT_TIME);
    calc.deps.add_with_source(
        missile_espec,
        AttrSpec::new(missile_espec.item_key, MISSILE_VELOCITY),
        affectee_aspec,
    );
    calc.deps
        .add_with_source(missile_espec, AttrSpec::new(ship_item_key, SHIP_RADIUS), affectee_aspec);
}
