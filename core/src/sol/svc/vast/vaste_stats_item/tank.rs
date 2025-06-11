use crate::{
    ac, ad,
    sol::{
        AttrVal, DmgKinds, ItemKey,
        svc::{calc::Calc, vast::Vast},
        uad::Uad,
    },
};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub structure: T,
}

impl Vast {
    pub(in crate::sol) fn get_item_hp(uad: &Uad, calc: &mut Calc, item_key: ItemKey) -> Option<StatTank<AttrVal>> {
        Some(StatTank {
            shield: calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::SHIELD_CAPACITY)?,
            armor: calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::ARMOR_HP)?,
            structure: calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::HP)?,
        })
    }
    pub(in crate::sol) fn get_item_resists(
        uad: &Uad,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Some(StatTank {
            shield: Vast::get_item_shield_resists(uad, calc, item_key)?,
            armor: Vast::get_item_armor_resists(uad, calc, item_key)?,
            structure: Vast::get_item_structure_resists(uad, calc, item_key)?,
        })
    }
    pub(in crate::sol) fn get_item_shield_resists(
        uad: &Uad,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            uad,
            calc,
            item_key,
            &ac::attrs::SHIELD_EM_DMG_RESONANCE,
            &ac::attrs::SHIELD_THERM_DMG_RESONANCE,
            &ac::attrs::SHIELD_KIN_DMG_RESONANCE,
            &ac::attrs::SHIELD_EXPL_DMG_RESONANCE,
        )
    }
    pub(in crate::sol) fn get_item_armor_resists(
        uad: &Uad,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            uad,
            calc,
            item_key,
            &ac::attrs::ARMOR_EM_DMG_RESONANCE,
            &ac::attrs::ARMOR_THERM_DMG_RESONANCE,
            &ac::attrs::ARMOR_KIN_DMG_RESONANCE,
            &ac::attrs::ARMOR_EXPL_DMG_RESONANCE,
        )
    }
    pub(in crate::sol) fn get_item_structure_resists(
        uad: &Uad,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            uad,
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
    uad: &Uad,
    calc: &mut Calc,
    item_key: ItemKey,
    em_a_attr_id: &ad::AAttrId,
    therm_a_attr_id: &ad::AAttrId,
    kin_a_attr_id: &ad::AAttrId,
    expl_a_attr_id: &ad::AAttrId,
) -> Option<DmgKinds<AttrVal>> {
    Some(DmgKinds {
        em: calc.get_item_attr_val_extra(uad, item_key, em_a_attr_id)?,
        thermal: calc.get_item_attr_val_extra(uad, item_key, therm_a_attr_id)?,
        kinetic: calc.get_item_attr_val_extra(uad, item_key, kin_a_attr_id)?,
        explosive: calc.get_item_attr_val_extra(uad, item_key, expl_a_attr_id)?,
    })
}
