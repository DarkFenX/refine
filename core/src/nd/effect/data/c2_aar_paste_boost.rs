// AAR paste boost in EVE does not change rep amount attribute. It seems to be applied by AAR effect
// when repairs actually happen. However, here we apply it to "extra" value of rep amount attribute
// for usability and simplicity of effect itself.

use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AEffect, AEffectId, AItem, AItemEffectData, AItemId, AState},
    def::{AttrVal, OF},
    misc::EffectSpec,
    nd::NEffect,
    rd::RAttrConsts,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AffectorValue, AggrMode, Calc, CalcOp, CustomAffectorValue,
            CustomAffectorValueKind, Location, ModifierKind, RawModifier,
        },
    },
    ud::{UItem, UItemKey},
    util::RMap,
};

const A_EFFECT_ID: AEffectId = ac::effects::AAR_PASTE_BOOST;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        calc_customizer: Some(calc_add_custom_modifier),
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
fn calc_add_custom_modifier(rmods: &mut Vec<RawModifier>, attr_consts: &RAttrConsts, espec: EffectSpec) {
    if let Some(armor_dmg_amount_key) = attr_consts.armor_dmg_amount
        && let Some(charged_armor_dmg_mult_key) = attr_consts.charged_armor_dmg_mult
    {
        let rmod = RawModifier {
            kind: ModifierKind::Local,
            affector_espec: espec,
            affector_value: AffectorValue::Custom(CustomAffectorValue {
                kind: CustomAffectorValueKind::AarRepAmount,
                affector_attr_key: Some(charged_armor_dmg_mult_key),
                affector_info_getter: get_affector_info,
                mod_val_getter: get_mod_val,
                item_add_reviser: Some(revise_on_item_add_removal),
                item_remove_reviser: Some(revise_on_item_add_removal),
            }),
            op: CalcOp::ExtraMul,
            aggr_mode: AggrMode::Stack,
            affectee_filter: AffecteeFilter::Direct(Location::Item),
            affectee_attr_key: armor_dmg_amount_key,
            ..
        };
        rmods.push(rmod);
    }
}

fn get_mod_val(calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
    // Return multiplier only if everything could be fetched successfully
    if let Some(charge_key) = ctx.u_data.items.get(espec.item_key).get_charge_key()
        && let ac::items::NANITE_REPAIR_PASTE = ctx.u_data.items.get(charge_key).get_type_id()
        && let Some(val) = calc.get_item_oattr_odogma(ctx, espec.item_key, ctx.ac().charged_armor_dmg_mult)
    {
        return Some(val);
    }
    Some(OF(1.0))
}

fn get_affector_info(ctx: SvcCtx, item_key: UItemKey) -> SmallVec<Affector, 1> {
    let mut info = SmallVec::new();
    if let Some(charged_armor_dmg_mult_key) = ctx.ac().charged_armor_dmg_mult {
        info.push(Affector {
            item_id: ctx.u_data.items.id_by_key(item_key),
            attr_id: Some(ctx.u_data.src.get_attr(charged_armor_dmg_mult_key).id.into()),
        });
    }
    info
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
