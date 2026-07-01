use smallvec::SmallVec;

use super::CalcCustomModStrength;
use crate::{
    api::AttrId,
    misc::EffectSpec,
    num::Value,
    rd::RAttrConsts,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AggrMode, Calc, CalcCustomModifier, CalcOp, Location, ModifierKind, RawModifier,
            modifier::ModStrength,
        },
    },
    ud::{UItem, UItemId},
};

pub(super) fn make_rmod(attr_consts: &RAttrConsts, espec: EffectSpec) -> Option<RawModifier> {
    attr_consts.radius?;
    Some(RawModifier {
        kind: ModifierKind::Local,
        affector_espec: espec,
        strength: ModStrength::Custom(CalcCustomModStrength {
            kind: CalcCustomModifier::MissileFlightTime,
            affector_attr_rid: Some(attr_consts.max_velocity?),
        }),
        op: CalcOp::ExtraAdd,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_attr_rid: attr_consts.explosion_delay?,
        ..
    })
}

pub(super) fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
    let ship_uid = ctx.u_data.get_item_fit_ship_uid(espec.item_uid)?;
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

pub(super) fn get_affector_info(ctx: SvcCtx, item_uid: UItemId) -> SmallVec<[Affector; 1]> {
    let mut info = SmallVec::new();
    if let Some(ship_uid) = ctx.u_data.get_item_fit_ship_uid(item_uid)
        && let Some(max_velocity_rid) = ctx.ac().max_velocity
        && let Some(radius_rid) = ctx.ac().radius
    {
        info.extend([
            Affector {
                item_id: ctx.u_data.items.ext_id_by_int_id(item_uid),
                attr_id: Some(AttrId::from_aid(
                    ctx.u_data.r_data.get_attr_by_rid(max_velocity_rid).aid,
                )),
            },
            // There is no dependency on modified ship radius, but we add it for informational
            // purposes nevertheless
            Affector {
                item_id: ctx.u_data.items.ext_id_by_int_id(ship_uid),
                attr_id: Some(AttrId::from_aid(ctx.u_data.r_data.get_attr_by_rid(radius_rid).aid)),
            },
        ]);
    };
    info
}

pub(super) fn revise_on_item_add_removal(ctx: SvcCtx, affector_uid: UItemId, changed_item: &UItem) -> bool {
    match changed_item {
        UItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_uid()) == ctx.u_data.items.get(affector_uid).get_fit_uid()
        }
        _ => false,
    }
}
