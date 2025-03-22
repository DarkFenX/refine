use smallvec::SmallVec;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId},
    util::StMap,
};

/// Adapted item type.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AItemKind {
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
    item_attrs: &StMap<AAttrId, AAttrVal>,
    item_effects: &StMap<AEffectId, AItemEffectData>,
) -> Option<AItemKind> {
    let mut kinds: SmallVec<AItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ac::itemcats::MODULE | ac::itemcats::STRUCTURE_MODULE => {
            if item_effects.contains_key(&ac::effects::HI_POWER) {
                kinds.push(AItemKind::ModuleHigh);
            }
            if item_effects.contains_key(&ac::effects::MED_POWER) {
                kinds.push(AItemKind::ModuleMid);
            }
            if item_effects.contains_key(&ac::effects::LO_POWER) {
                kinds.push(AItemKind::ModuleLow);
            }
            if item_effects.contains_key(&ac::effects::RIG_SLOT) {
                kinds.push(AItemKind::Rig);
            }
            if item_effects.contains_key(&ac::effects::SERVICE_SLOT) {
                kinds.push(AItemKind::Service);
            }
        }
        // Ships and structures
        ac::itemcats::SHIP | ac::itemcats::STRUCTURE => kinds.push(AItemKind::Ship),
        // Implants and boosters
        ac::itemcats::IMPLANT => {
            if item_attrs.contains_key(&ac::attrs::BOOSTERNESS) {
                kinds.push(AItemKind::Booster);
            }
            if item_attrs.contains_key(&ac::attrs::IMPLANTNESS) {
                kinds.push(AItemKind::Implant);
            }
        }
        // Other items
        ac::itemcats::CHARGE => kinds.push(AItemKind::Charge),
        ac::itemcats::DRONE => kinds.push(AItemKind::Drone),
        ac::itemcats::FIGHTER => kinds.push(AItemKind::Fighter),
        ac::itemcats::SKILL => kinds.push(AItemKind::Skill),
        ac::itemcats::SUBSYSTEM => kinds.push(AItemKind::Subsystem),
        _ => (),
    }
    match item_grp_id {
        ac::itemgrps::CHARACTER => kinds.push(AItemKind::Character),
        ac::itemgrps::SHIP_MODIFIER => kinds.push(AItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}
