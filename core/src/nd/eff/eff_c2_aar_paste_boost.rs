use smallvec::{SmallVec, smallvec};

use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectId, AItem, AItemEffectData, AItemId, AState},
    def::{AttrVal, OF},
    misc::EffectSpec,
    nd::{NEffect, NEffectHc},
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

const A_EFFECT_ID: AEffectId = ac::effects::AAR_PASTE_BOOST;
const AAR_MULTIPLIER: AAttrId = ac::attrs::CHARGED_ARMOR_DMG_MULT;

pub(super) fn mk_n_effect() -> NEffect {
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
        state: AState::Offline,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas
                .contains_key(&ac::effects::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER)
    }) {
        a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
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
            kind: CustomAffectorValueKind::AarRepAmount,
            affector_attr_id: Some(AAR_MULTIPLIER),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            item_add_reviser: Some(revise_on_item_add_removal),
            item_remove_reviser: Some(revise_on_item_add_removal),
        }),
        op: Op::ExtraMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_attr_id: ac::attrs::ARMOR_DMG_AMOUNT,
        ..
    };
    rmods.push(rmod);
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let item = ctx.u_data.items.get(espec.item_key);
    match item.get_charge_key() {
        Some(charge_key) => {
            let charge = ctx.u_data.items.get(charge_key);
            match charge.get_type_id() {
                ac::items::NANITE_REPAIR_PASTE => {
                    match calc.get_item_attr_val_full(ctx, espec.item_key, &AAR_MULTIPLIER) {
                        Ok(sol_attr) => Some(sol_attr.dogma),
                        // Can't fetch multiplier attr - no extra reps
                        Err(_) => Some(OF(1.0)),
                    }
                }
                // Different charge - no extra reps
                _ => Some(OF(1.0)),
            }
        }
        // No charge - no extra reps
        None => Some(OF(1.0)),
    }
}

fn get_affector_info(ctx: SvcCtx, item_key: UItemKey) -> SmallVec<AffectorInfo, 1> {
    smallvec![AffectorInfo {
        item_id: ctx.u_data.items.id_by_key(item_key),
        attr_id: Some(AAR_MULTIPLIER)
    }]
}

fn revise_on_item_add_removal(
    ctx: SvcCtx,
    affector_key: UItemKey,
    changed_key: UItemKey,
    changed_item: &UItem,
) -> bool {
    match ctx.u_data.items.get(affector_key).get_charge_key() {
        Some(charge_key) => changed_key == charge_key && changed_item.get_type_id() == ac::items::NANITE_REPAIR_PASTE,
        // Not chargeable item, or no charge on AAR -> not changing anything
        None => false,
    }
}
