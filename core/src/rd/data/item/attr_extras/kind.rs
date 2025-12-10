use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AAttrVal, AItemCatId, AItemGrpId},
    rd::{RAttrConsts, RAttrKey, REffectConsts, REffectKey, RItemEffectData},
    util::RMap,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    Fighter,
    Implant,
    ModuleHigh,
    ModuleMid,
    ModuleLow,
    Rig,
    Service,
    Ship,
    Skill,
    Stance,
    Subsystem,
}

pub(super) fn get_item_kind(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    item_effects: &RMap<REffectKey, RItemEffectData>,
    attr_consts: &RAttrConsts,
    effect_consts: &REffectConsts,
) -> Option<RItemKind> {
    let mut kinds: SmallVec<RItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if let Some(effect_key) = effect_consts.hi_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(RItemKind::ModuleHigh);
            }
            if let Some(effect_key) = effect_consts.med_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(RItemKind::ModuleMid);
            }
            if let Some(effect_key) = effect_consts.lo_power
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(RItemKind::ModuleLow);
            }
            if let Some(effect_key) = effect_consts.rig_slot
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(RItemKind::Rig);
            }
            if let Some(effect_key) = effect_consts.service_slot
                && item_effects.contains_key(&effect_key)
            {
                kinds.push(RItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(RItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if let Some(attr_key) = attr_consts.boosterness
                && item_attrs.contains_key(&attr_key)
            {
                kinds.push(RItemKind::Booster);
            }
            if let Some(attr_key) = attr_consts.implantness
                && item_attrs.contains_key(&attr_key)
            {
                kinds.push(RItemKind::Implant);
            }
        }
        // Other items
        ac::itemcats::CHARGE => kinds.push(RItemKind::Charge),
        ac::itemcats::DRONE => kinds.push(RItemKind::Drone),
        ac::itemcats::FIGHTER => kinds.push(RItemKind::Fighter),
        ac::itemcats::SKILL => kinds.push(RItemKind::Skill),
        ac::itemcats::SUBSYSTEM => kinds.push(RItemKind::Subsystem),
        _ => (),
    }
    match item_grp_id {
        ac::itemgrps::CHARACTER => kinds.push(RItemKind::Character),
        ac::itemgrps::SHIP_MODIFIER => kinds.push(RItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}
