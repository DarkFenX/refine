use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AItemCatId, AItemGrpId},
    misc::{ItemKind, Value},
    rd::{RAttrConsts, RAttrId, REffectConsts, REffectId, RItemEffectData},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_item_kind(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrId, Value>,
    item_effects: &RMap<REffectId, RItemEffectData>,
    attr_consts: &RAttrConsts,
    effect_consts: &REffectConsts,
) -> Option<ItemKind> {
    let mut kinds: SmallVec<ItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if let Some(effect_rid) = effect_consts.hi_power
                && item_effects.contains_key(&effect_rid)
            {
                kinds.push(ItemKind::ModuleHigh);
            }
            if let Some(effect_rid) = effect_consts.med_power
                && item_effects.contains_key(&effect_rid)
            {
                kinds.push(ItemKind::ModuleMid);
            }
            if let Some(effect_rid) = effect_consts.lo_power
                && item_effects.contains_key(&effect_rid)
            {
                kinds.push(ItemKind::ModuleLow);
            }
            if let Some(effect_rid) = effect_consts.rig_slot
                && item_effects.contains_key(&effect_rid)
            {
                kinds.push(ItemKind::Rig);
            }
            if let Some(effect_rid) = effect_consts.service_slot
                && item_effects.contains_key(&effect_rid)
            {
                kinds.push(ItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(ItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if let Some(attr_rid) = attr_consts.boosterness
                && item_attrs.contains_key(&attr_rid)
            {
                kinds.push(ItemKind::Booster);
            }
            if let Some(attr_rid) = attr_consts.implantness
                && item_attrs.contains_key(&attr_rid)
            {
                kinds.push(ItemKind::Implant);
            }
        }
        // Other items
        ac::itemcats::CHARGE => kinds.push(ItemKind::Charge),
        ac::itemcats::DRONE => kinds.push(ItemKind::Drone),
        ac::itemcats::FIGHTER => kinds.push(ItemKind::Fighter),
        ac::itemcats::SKILL => kinds.push(ItemKind::Skill),
        ac::itemcats::SUBSYSTEM => kinds.push(ItemKind::Subsystem),
        _ => (),
    }
    match item_grp_id {
        AItemGrpId::CHARACTER => kinds.push(ItemKind::Character),
        AItemGrpId::SHIP_MODIFIER => kinds.push(ItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}
