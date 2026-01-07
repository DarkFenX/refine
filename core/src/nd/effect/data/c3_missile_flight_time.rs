// In EVE, missiles are launched from center of an attacking ship. Ships have non-zero radius, and
// overview distance is calculated from surface to surface. To make missile range roughly match to
// their theoretical range, CCP added hidden flight time bonus, which depends on radius of the
// attacking ship. This effect implements it.

use smallvec::SmallVec;

use crate::{
    ad::{AEffect, AEffectCatId, AEffectId, AItem, AItemEffectData, AItemId, AState},
    api::AttrId,
    misc::{EffectSpec, Value},
    nd::{NEffect, effect::data::shared::util::get_item_fit_ship_uid},
    rd::RAttrConsts,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AffectorValue, AggrMode, Calc, CalcOp, CustomAffectorValue,
            CustomAffectorValueKind, Location, ModifierKind, RawModifier,
        },
    },
    ud::{UItem, UItemId},
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::MISSILE_FLIGHT_TIME;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        calc_customizer: Some(calc_add_custom_modifier),
        ..
    }
}

// ADG customizations
fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::PASSIVE,
        state: AState::Disabled,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for item in a_items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&AEffectId::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&AEffectId::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&AEffectId::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&AEffectId::DOT_MISSILE_LAUNCHING)
    }) {
        item.effect_datas.insert(EFFECT_AID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}

// Calc customizations
fn calc_add_custom_modifier(rmods: &mut Vec<RawModifier>, attr_consts: &RAttrConsts, espec: EffectSpec) {
    if let Some(max_velocity_rid) = attr_consts.max_velocity
        && let Some(explosion_delay_rid) = attr_consts.explosion_delay
        && attr_consts.radius.is_some()
    {
        let rmod = RawModifier {
            kind: ModifierKind::Local,
            affector_espec: espec,
            affector_value: AffectorValue::Custom(CustomAffectorValue {
                kind: CustomAffectorValueKind::MissileFlightTime,
                affector_attr_rid: Some(max_velocity_rid),
                affector_info_getter: get_affector_info,
                mod_val_getter: get_mod_val,
                item_add_reviser: Some(revise_on_item_add_removal),
                item_remove_reviser: Some(revise_on_item_add_removal),
            }),
            op: CalcOp::ExtraAdd,
            aggr_mode: AggrMode::Stack,
            affectee_filter: AffecteeFilter::Direct(Location::Item),
            affectee_attr_rid: explosion_delay_rid,
            ..
        };
        rmods.push(rmod);
    }
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
    let ship_uid = get_item_fit_ship_uid(ctx, espec.item_uid)?;
    let missile_velocity = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().max_velocity)?;
    let ship_radius = ctx.u_data.items.get(ship_uid).get_direct_radius();
    // Missile flight time is stored in milliseconds
    let val = ship_radius / missile_velocity * Value::THOUSAND;
    if !val.is_finite() {
        return None;
    }
    // No need to register dependencies here, because missile velocity attribute is getting cleared
    // the regular modifier way, and ship radius is taken unmodified intentionally. Since it is
    // taken unmodified, it should stay as-is, and applied modification doesn't need to be cleared
    // up whenever modified value changes
    Some(val)
}

fn get_affector_info(ctx: SvcCtx, item_uid: UItemId) -> SmallVec<Affector, 1> {
    let mut info = SmallVec::new();
    if let Some(ship_uid) = get_item_fit_ship_uid(ctx, item_uid)
        && let Some(max_velocity_rid) = ctx.ac().max_velocity
        && let Some(radius_rid) = ctx.ac().radius
    {
        info.extend([
            Affector {
                item_id: ctx.u_data.items.xid_by_iid(item_uid),
                attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(max_velocity_rid).aid)),
            },
            // There is no dependency on modified ship radius, but we add it for informational
            // purposes nevertheless
            Affector {
                item_id: ctx.u_data.items.xid_by_iid(ship_uid),
                attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(radius_rid).aid)),
            },
        ]);
    };
    info
}

fn revise_on_item_add_removal(ctx: SvcCtx, affector_uid: UItemId, _changed_uid: UItemId, changed_item: &UItem) -> bool {
    match changed_item {
        UItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_uid()) == ctx.u_data.items.get(affector_uid).get_fit_uid()
        }
        _ => false,
    }
}
