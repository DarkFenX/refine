use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemKey,
        svc::{EffectSpec, calc::Calc, get_resist_mult_val},
        uad::Uad,
    },
};

pub(in crate::sol::svc::vast) fn get_effect_local_shield_rep_amount(
    uad: &Uad,
    calc: &mut Calc,
    espec: &EffectSpec,
) -> Option<AttrVal> {
    let mut amount = match espec.a_effect_id {
        ac::effects::FUELED_SHIELD_BOOSTING => {
            calc.get_item_attr_val_extra(uad, espec.item_key, &ac::attrs::SHIELD_BONUS)?
        }
        _ => return None,
    };
    // If rep target is defined and has less than repped amount HP, limit by total HP
    if let Some(hp) = get_ship_attr(uad, calc, espec.item_key, &ac::attrs::SHIELD_CAPACITY) {
        amount = amount.min(hp);
    }
    Some(amount)
}

pub(in crate::sol::svc::vast) fn get_effect_local_armor_rep_amount(
    uad: &Uad,
    calc: &mut Calc,
    espec: &EffectSpec,
) -> Option<AttrVal> {
    let mut amount = match espec.a_effect_id {
        ac::effects::FUELED_ARMOR_REPAIR => {
            calc.get_item_attr_val_extra(uad, espec.item_key, &ac::attrs::ARMOR_DMG_AMOUNT)?
        }
        _ => return None,
    };
    // If rep target is defined and has less than repped amount HP, limit by total HP
    if let Some(hp) = get_ship_attr(uad, calc, espec.item_key, &ac::attrs::ARMOR_HP) {
        amount = amount.min(hp);
    }
    Some(amount)
}

pub(in crate::sol::svc::vast) fn get_effect_remote_shield_rep_amount(
    uad: &Uad,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_item_key: Option<ItemKey>,
) -> Option<AttrVal> {
    let mut amount = match projector_espec.a_effect_id {
        ac::effects::SHIP_MODULE_ARSR => {
            calc.get_item_attr_val_extra(uad, projector_espec.item_key, &ac::attrs::SHIELD_BONUS)?
        }
        _ => return None,
    };
    if let Some(projectee_item_key) = projectee_item_key {
        // RR impedance reduction
        if let Some(rr_mult) = get_resist_mult_val(uad, calc, projector_espec, projectee_item_key) {
            amount * -rr_mult;
        }
        // If rep target has less than repped amount HP, limit by target HP
        if let Some(hp) = calc.get_item_attr_val_extra(uad, projectee_item_key, &ac::attrs::SHIELD_CAPACITY) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}

pub(in crate::sol::svc::vast) fn get_effect_remote_armor_rep_amount(
    uad: &Uad,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_item_key: Option<ItemKey>,
) -> Option<AttrVal> {
    let mut amount = match projector_espec.a_effect_id {
        ac::effects::SHIP_MODULE_ARAR => {
            calc.get_item_attr_val_extra(uad, projector_espec.item_key, &ac::attrs::ARMOR_DMG_AMOUNT)?
        }
        _ => return None,
    };
    if let Some(projectee_item_key) = projectee_item_key {
        // RR impedance reduction
        if let Some(rr_mult) = get_resist_mult_val(uad, calc, projector_espec, projectee_item_key) {
            amount * -rr_mult;
        }
        // If rep target has less than repped amount HP, limit by target HP
        if let Some(hp) = calc.get_item_attr_val_extra(uad, projectee_item_key, &ac::attrs::ARMOR_HP) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}

fn get_ship_attr(uad: &Uad, calc: &mut Calc, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> Option<AttrVal> {
    let fit_key = uad.items.get(item_key).get_fit_key()?;
    let ship_key = uad.fits.get(fit_key).ship?;
    calc.get_item_attr_val_extra(uad, ship_key, a_attr_id)
}
