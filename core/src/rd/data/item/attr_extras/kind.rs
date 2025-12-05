use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId},
    rd::REffectKey,
    src::Src,
    util::RMap,
};

/// Adapted item type.
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

pub(super) fn get_item_kind_initial(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<AAttrId, AAttrVal>,
    item_effects: &RMap<AEffectId, AItemEffectData>,
) -> Option<RItemKind> {
    let mut kinds: SmallVec<RItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if item_effects.contains_key(&ac::effects::HI_POWER) {
                kinds.push(RItemKind::ModuleHigh);
            }
            if item_effects.contains_key(&ac::effects::MED_POWER) {
                kinds.push(RItemKind::ModuleMid);
            }
            if item_effects.contains_key(&ac::effects::LO_POWER) {
                kinds.push(RItemKind::ModuleLow);
            }
            if item_effects.contains_key(&ac::effects::RIG_SLOT) {
                kinds.push(RItemKind::Rig);
            }
            if item_effects.contains_key(&ac::effects::SERVICE_SLOT) {
                kinds.push(RItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(RItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if item_attrs.contains_key(&ac::attrs::BOOSTERNESS) {
                kinds.push(RItemKind::Booster);
            }
            if item_attrs.contains_key(&ac::attrs::IMPLANTNESS) {
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

pub(super) fn get_item_kind_inherited(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<AAttrId, AAttrVal>,
    item_effects: &RMap<REffectKey, AItemEffectData>,
    src: &Src,
) -> Option<RItemKind> {
    let mut kinds: SmallVec<RItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if let Some(hi_slot_effect_key) = src.get_effect_consts().hi_slot
                && item_effects.contains_key(&hi_slot_effect_key)
            {
                kinds.push(RItemKind::ModuleHigh);
            }
            if let Some(mid_slot_effect_key) = src.get_effect_consts().mid_slot
                && item_effects.contains_key(&mid_slot_effect_key)
            {
                kinds.push(RItemKind::ModuleMid);
            }
            if let Some(low_slot_effect_key) = src.get_effect_consts().low_slot
                && item_effects.contains_key(&low_slot_effect_key)
            {
                kinds.push(RItemKind::ModuleLow);
            }
            if let Some(rig_slot_effect_key) = src.get_effect_consts().rig_slot
                && item_effects.contains_key(&rig_slot_effect_key)
            {
                kinds.push(RItemKind::Rig);
            }
            if let Some(svc_slot_effect_key) = src.get_effect_consts().svc_slot
                && item_effects.contains_key(&svc_slot_effect_key)
            {
                kinds.push(RItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(RItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if item_attrs.contains_key(&ac::attrs::BOOSTERNESS) {
                kinds.push(RItemKind::Booster);
            }
            if item_attrs.contains_key(&ac::attrs::IMPLANTNESS) {
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
