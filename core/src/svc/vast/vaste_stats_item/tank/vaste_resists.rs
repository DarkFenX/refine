use super::shared::item_key_check;
use crate::{
    ac, ad,
    def::{AttrVal, OF},
    misc::DmgKinds,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_resists(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<DmgKinds<AttrVal>>, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_resists_unchecked(ctx, calc, item_key))
    }
    pub(super) fn get_stat_item_resists_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> StatTank<DmgKinds<AttrVal>> {
        StatTank {
            shield: Vast::get_item_shield_resists(ctx, calc, item_key),
            armor: Vast::get_item_armor_resists(ctx, calc, item_key),
            hull: Vast::get_item_hull_resists(ctx, calc, item_key),
        }
    }
    fn get_item_shield_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::SHIELD_EM_DMG_RESONANCE,
            &ac::attrs::SHIELD_THERM_DMG_RESONANCE,
            &ac::attrs::SHIELD_KIN_DMG_RESONANCE,
            &ac::attrs::SHIELD_EXPL_DMG_RESONANCE,
        )
    }
    fn get_item_armor_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::ARMOR_EM_DMG_RESONANCE,
            &ac::attrs::ARMOR_THERM_DMG_RESONANCE,
            &ac::attrs::ARMOR_KIN_DMG_RESONANCE,
            &ac::attrs::ARMOR_EXPL_DMG_RESONANCE,
        )
    }
    fn get_item_hull_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::EM_DMG_RESONANCE,
            &ac::attrs::THERM_DMG_RESONANCE,
            &ac::attrs::KIN_DMG_RESONANCE,
            &ac::attrs::EXPL_DMG_RESONANCE,
        )
    }
}

fn get_item_layer_resists(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    em_a_attr_id: &ad::AAttrId,
    therm_a_attr_id: &ad::AAttrId,
    kin_a_attr_id: &ad::AAttrId,
    expl_a_attr_id: &ad::AAttrId,
) -> DmgKinds<AttrVal> {
    DmgKinds {
        em: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, em_a_attr_id).unwrap(),
        thermal: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, therm_a_attr_id).unwrap(),
        kinetic: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, kin_a_attr_id).unwrap(),
        explosive: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, expl_a_attr_id).unwrap(),
    }
}
