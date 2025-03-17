use smallvec::SmallVec;

use crate::{
    ad::AItemEffectData,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId},
    ec,
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
    item_grp_id: EItemGrpId,
    item_cat_id: EItemCatId,
    item_attrs: &StMap<EAttrId, AttrVal>,
    item_effects: &StMap<EEffectId, AItemEffectData>,
) -> Option<AItemKind> {
    let mut kinds: SmallVec<AItemKind, 1> = SmallVec::new();
    match item_cat_id {
        // Ship & structure modules
        ec::itemcats::MODULE | ec::itemcats::STRUCTURE_MODULE => {
            if item_effects.contains_key(&ec::effects::HI_POWER) {
                kinds.push(AItemKind::ModuleHigh);
            }
            if item_effects.contains_key(&ec::effects::MED_POWER) {
                kinds.push(AItemKind::ModuleMid);
            }
            if item_effects.contains_key(&ec::effects::LO_POWER) {
                kinds.push(AItemKind::ModuleLow);
            }
            if item_effects.contains_key(&ec::effects::RIG_SLOT) {
                kinds.push(AItemKind::Rig);
            }
            if item_effects.contains_key(&ec::effects::SERVICE_SLOT) {
                kinds.push(AItemKind::Service);
            }
        }
        // Ships and structures
        ec::itemcats::SHIP | ec::itemcats::STRUCTURE => kinds.push(AItemKind::Ship),
        // Implants and boosters
        ec::itemcats::IMPLANT => {
            if item_attrs.contains_key(&ec::attrs::BOOSTERNESS) {
                kinds.push(AItemKind::Booster);
            }
            if item_attrs.contains_key(&ec::attrs::IMPLANTNESS) {
                kinds.push(AItemKind::Implant);
            }
        }
        // Other items
        ec::itemcats::CHARGE => kinds.push(AItemKind::Charge),
        ec::itemcats::DRONE => kinds.push(AItemKind::Drone),
        ec::itemcats::FIGHTER => kinds.push(AItemKind::Fighter),
        ec::itemcats::SKILL => kinds.push(AItemKind::Skill),
        ec::itemcats::SUBSYSTEM => kinds.push(AItemKind::Subsystem),
        _ => (),
    }
    match item_grp_id {
        ec::itemgrps::CHARACTER => kinds.push(AItemKind::Character),
        ec::itemgrps::SHIP_MODIFIER => kinds.push(AItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}
