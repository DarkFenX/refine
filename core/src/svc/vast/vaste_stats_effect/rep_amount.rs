use crate::{
    ac,
    def::{AttrVal, ItemKey},
    misc::EffectSpec,
    svc::{SvcCtx, calc::Calc, get_resist_mult_val},
};

pub(in crate::svc::vast) fn get_effect_remote_shield_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_item_key: Option<ItemKey>,
) -> Option<AttrVal> {
    let mut amount = match projector_espec.a_effect_id {
        ac::effects::SHIP_MODULE_RASB => {
            calc.get_item_attr_val_extra(ctx, projector_espec.item_key, &ac::attrs::SHIELD_BONUS)?
        }
        _ => return None,
    };
    if let Some(projectee_item_key) = projectee_item_key {
        // RR impedance reduction
        if let Some(rr_mult) = get_resist_mult_val(ctx, calc, projector_espec, projectee_item_key) {
            amount *= rr_mult;
        }
        // If rep target has less than repped amount HP, limit by target HP
        if let Some(hp) = calc.get_item_attr_val_extra(ctx, projectee_item_key, &ac::attrs::SHIELD_CAPACITY) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}

pub(in crate::svc::vast) fn get_effect_remote_armor_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_item_key: Option<ItemKey>,
) -> Option<AttrVal> {
    let mut amount = match projector_espec.a_effect_id {
        ac::effects::SHIP_MODULE_RAAR => {
            calc.get_item_attr_val_extra(ctx, projector_espec.item_key, &ac::attrs::ARMOR_DMG_AMOUNT)?
        }
        _ => return None,
    };
    if let Some(projectee_item_key) = projectee_item_key {
        // RR impedance reduction
        if let Some(rr_mult) = get_resist_mult_val(ctx, calc, projector_espec, projectee_item_key) {
            amount *= rr_mult;
        }
        // If rep target has less than repped amount HP, limit by target HP
        if let Some(hp) = calc.get_item_attr_val_extra(ctx, projectee_item_key, &ac::attrs::ARMOR_HP) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}
