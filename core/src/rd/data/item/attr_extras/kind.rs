use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AAttrVal, AItemCatId, AItemGrpId},
    misc::ItemKind,
    rd::{RAttrConsts, RAttrKey, REffectConsts, REffectKey, RItemEffectData},
    util::RMap,
};

pub(super) fn get_item_kind(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    item_effects: &RMap<REffectKey, RItemEffectData>,
    attr_consts: &RAttrConsts,
    effect_consts: &REffectConsts,
) -> Option<ItemKind> {
    let mut kinds: SmallVec<ItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if let Some(effect_key) = effect_consts.hi_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(ItemKind::ModuleHigh);
            }
            if let Some(effect_key) = effect_consts.med_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(ItemKind::ModuleMid);
            }
            if let Some(effect_key) = effect_consts.lo_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(ItemKind::ModuleLow);
            }
            if let Some(effect_key) = effect_consts.rig_slot
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(ItemKind::Rig);
            }
            if let Some(effect_key) = effect_consts.service_slot
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(ItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(ItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if let Some(attr_key) = attr_consts.boosterness
                && item_attrs.contains_key(&attr_key)
            {
                kinds.push(ItemKind::Booster);
            }
            if let Some(attr_key) = attr_consts.implantness
                && item_attrs.contains_key(&attr_key)
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
        ac::itemgrps::CHARACTER => kinds.push(ItemKind::Character),
        ac::itemgrps::SHIP_MODIFIER => kinds.push(ItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}
