use smallvec::{SmallVec, smallvec};

use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    ntt::{NttEffect, NttEffectRt},
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AffectorValue, AggrMode, Calc, CustomAffectorValue, Location, ModifierKind,
            Op, RawModifier,
        },
    },
    uad::UadItem,
};

const A_EFFECT_ID: ad::AEffectId = ac::effects::AAR_PASTE_BOOST;
const AAR_MULTIPLIER: ad::AAttrId = ac::attrs::CHARGED_ARMOR_DMG_MULT;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(adg_add_custom_effect),
        rt: NttEffectRt {
            calc_custom_fn: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

// ADG customizations
fn adg_add_custom_effect(a_data: &mut ad::AData) {
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
        v.effect_datas.contains_key(&ac::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas.contains_key(&ac::effects::SHIP_MODULE_RAAR)
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
            affector_a_attr_id: Some(AAR_MULTIPLIER),
            affector_info_getter: get_affector_info,
            mod_val_getter: get_mod_val,
            item_add_reviser: Some(revise_on_item_add_removal),
            item_remove_reviser: Some(revise_on_item_add_removal),
        }),
        op: Op::ExtraMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_a_attr_id: ac::attrs::ARMOR_DMG_AMOUNT,
        ..
    };
    rmods.push(rmod);
}

fn get_mod_val(calc: &mut Calc, ctx: &SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    let item = ctx.uad.items.get(espec.item_key);
    match item.get_charge_item_key() {
        Some(charge_key) => {
            let charge = ctx.uad.items.get(charge_key);
            match charge.get_a_item_id() {
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

fn get_affector_info(ctx: &SvcCtx, item_key: ItemKey) -> SmallVec<AffectorInfo, 1> {
    smallvec![AffectorInfo {
        item_id: ctx.uad.items.id_by_key(item_key),
        attr_id: Some(AAR_MULTIPLIER)
    }]
}

fn revise_on_item_add_removal(
    ctx: &SvcCtx,
    affector_key: ItemKey,
    changed_item_key: ItemKey,
    changed_item: &UadItem,
) -> bool {
    match ctx.uad.items.get(affector_key) {
        UadItem::Module(module) => match module.get_charge_item_key() {
            Some(charge_key) => {
                changed_item_key == charge_key && changed_item.get_a_item_id() == ac::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
