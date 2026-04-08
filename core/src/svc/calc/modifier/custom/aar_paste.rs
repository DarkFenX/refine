use smallvec::SmallVec;

use super::CalcCustomModStrength;
use crate::{
    ad::AItemId,
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
    Some(RawModifier {
        kind: ModifierKind::Local,
        affector_espec: espec,
        strength: ModStrength::Custom(CalcCustomModStrength {
            kind: CalcCustomModifier::AarPaste,
            affector_attr_rid: Some(attr_consts.charged_armor_dmg_mult?),
        }),
        op: CalcOp::ExtraMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_attr_rid: attr_consts.armor_dmg_amount?,
        ..
    })
}

pub(super) fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
    // Return multiplier only if everything could be fetched successfully
    if let Some(charge_uid) = ctx.u_data.items.get(espec.item_uid).get_charge_uid()
        && let AItemId::NANITE_REPAIR_PASTE = ctx.u_data.items.get(charge_uid).get_type_aid()
        && let Some(val) = calc.get_item_oattr_odogma(ctx, espec.item_uid, ctx.ac().charged_armor_dmg_mult)
    {
        return Some(val);
    }
    Some(Value::ONE)
}

pub(super) fn get_affector_info(ctx: SvcCtx, item_uid: UItemId) -> SmallVec<Affector, 1> {
    let mut info = SmallVec::new();
    if let Some(mult_attr_rid) = ctx.ac().charged_armor_dmg_mult {
        info.push(Affector {
            item_id: ctx.u_data.items.xid_by_iid(item_uid),
            attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(mult_attr_rid).aid)),
        });
    }
    info
}

pub(super) fn revise_on_item_add_removal(
    ctx: SvcCtx,
    affector_uid: UItemId,
    changed_uid: UItemId,
    changed_item: &UItem,
) -> bool {
    match ctx.u_data.items.get(affector_uid).get_charge_uid() {
        Some(charge_uid) => changed_uid == charge_uid && changed_item.get_type_aid() == AItemId::NANITE_REPAIR_PASTE,
        // Not chargeable item, or no charge on AAR -> not changing anything
        None => false,
    }
}
