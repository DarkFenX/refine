use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        AttrVal, DmgKinds, ItemKey,
        svc::{
            calc::Calc,
            vast::{
                Vast,
                shared::{get_effect_local_armor_rep_amount, get_effect_local_shield_rep_amount},
            },
        },
        uad::{Uad, item::UadItem},
    },
};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub structure: T,
}

pub struct StatLayerHp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
}

impl Vast {
    pub(in crate::sol) fn get_item_hp(uad: &Uad, calc: &mut Calc, item_key: ItemKey) -> Option<StatTank<StatLayerHp>> {
        let (local_ancil_shield, local_ancil_armor) = match uad.items.get(item_key) {
            UadItem::Ship(_) => (OF(0.0), OF(0.0)),
            _ => (OF(0.0), OF(0.0)),
        };
        let shield_buffer = calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::SHIELD_CAPACITY)?;
        let armor_buffer = calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::ARMOR_HP)?;
        let structure_buffer = calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::HP)?;
        Some(StatTank {
            shield: StatLayerHp {
                buffer: shield_buffer,
                ancil_local: local_ancil_shield,
                ancil_remote: OF(0.0),
            },
            armor: StatLayerHp {
                buffer: armor_buffer,
                ancil_local: local_ancil_armor,
                ancil_remote: OF(0.0),
            },
            structure: StatLayerHp {
                buffer: structure_buffer,
                ancil_local: OF(0.0),
                ancil_remote: OF(0.0),
            },
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
