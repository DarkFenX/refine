use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::DmgKinds,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{StatTank, Vast},
    },
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_resists(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Some(StatTank {
            shield: Vast::get_item_shield_resists(ctx, calc, item_key)?,
            armor: Vast::get_item_armor_resists(ctx, calc, item_key)?,
            hull: Vast::get_item_hull_resists(ctx, calc, item_key)?,
        })
    }
    fn get_item_shield_resists(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<DmgKinds<AttrVal>> {
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
    fn get_item_armor_resists(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<DmgKinds<AttrVal>> {
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
    fn get_item_hull_resists(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<DmgKinds<AttrVal>> {
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
    item_key: ItemKey,
    em_a_attr_id: &ad::AAttrId,
    therm_a_attr_id: &ad::AAttrId,
    kin_a_attr_id: &ad::AAttrId,
    expl_a_attr_id: &ad::AAttrId,
) -> Option<DmgKinds<AttrVal>> {
    Some(DmgKinds {
        em: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, em_a_attr_id)?,
        thermal: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, therm_a_attr_id)?,
        kinetic: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, kin_a_attr_id)?,
        explosive: OF(1.0) - calc.get_item_attr_val_extra(ctx, item_key, expl_a_attr_id)?,
    })
}
