use smallvec::SmallVec;

use crate::{
    ad::AItemEffectData,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, OF},
    ec,
    util::StMap,
};

/// Adapted item type.
#[derive(Copy, Clone)]
pub enum AItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad(AFighterKind),
    Implant,
    Module(AModRack, AShipKind),
    Mutator,
    Rig(AShipKind),
    Ship(AShipKind),
    Skill,
    Stance,
    Subsystem,
}

/// Adapted ship type.
#[derive(Copy, Clone)]
pub enum AShipKind {
    Ship,
    CapitalShip,
    Structure,
}

/// Adapted module rack.
#[derive(Copy, Clone)]
pub enum AModRack {
    High,
    Mid,
    Low,
}

/// Adapted fighter squad type.
#[derive(Copy, Clone)]
pub enum AFighterKind {
    Support,
    Light,
    Heavy,
    StandupSupport,
    StandupLight,
    StandupHeavy,
}

pub(super) fn get_item_kind(
    grp_id: EItemGrpId,
    cat_id: EItemCatId,
    attrs: &StMap<EAttrId, AttrVal>,
    effects: &StMap<EEffectId, AItemEffectData>,
    srqs: &StMap<EItemId, SkillLevel>,
) -> Option<AItemKind> {
    let mut kinds: SmallVec<AItemKind, 1> = SmallVec::new();
    match cat_id {
        // Ship & structure modules
        ec::itemcats::MODULE => {
            let ship_kind = match attrs.get(&ec::attrs::VOLUME) {
                Some(&volume) => match volume <= OF(3500.0) {
                    true => AShipKind::Ship,
                    false => AShipKind::CapitalShip,
                },
                None => AShipKind::Ship,
            };
            if effects.contains_key(&ec::effects::HI_POWER) {
                kinds.push(AItemKind::Module(AModRack::High, ship_kind));
            }
            if effects.contains_key(&ec::effects::MED_POWER) {
                kinds.push(AItemKind::Module(AModRack::Mid, ship_kind));
            }
            if effects.contains_key(&ec::effects::LO_POWER) {
                kinds.push(AItemKind::Module(AModRack::Low, ship_kind));
            }
            if effects.contains_key(&ec::effects::RIG_SLOT) {
                kinds.push(AItemKind::Rig(AShipKind::Ship));
            }
        }
        ec::itemcats::STRUCTURE_MODULE => {
            if effects.contains_key(&ec::effects::HI_POWER) {
                kinds.push(AItemKind::Module(AModRack::High, AShipKind::Structure));
            }
            if effects.contains_key(&ec::effects::MED_POWER) {
                kinds.push(AItemKind::Module(AModRack::Mid, AShipKind::Structure));
            }
            if effects.contains_key(&ec::effects::LO_POWER) {
                kinds.push(AItemKind::Module(AModRack::Low, AShipKind::Structure));
            }
            if effects.contains_key(&ec::effects::RIG_SLOT) {
                kinds.push(AItemKind::Rig(AShipKind::Structure));
            }
        }
        // Ships and structures
        ec::itemcats::SHIP => match srqs.contains_key(&ec::items::CAPITAL_SHIPS) {
            true => kinds.push(AItemKind::Ship(AShipKind::CapitalShip)),
            false => kinds.push(AItemKind::Ship(AShipKind::Ship)),
        },
        ec::itemcats::STRUCTURE => kinds.push(AItemKind::Ship(AShipKind::Structure)),
        // Implants and boosters
        ec::itemcats::IMPLANT => {
            if attrs.contains_key(&ec::attrs::BOOSTERNESS) {
                kinds.push(AItemKind::Booster);
            }
            if attrs.contains_key(&ec::attrs::IMPLANTNESS) {
                kinds.push(AItemKind::Implant);
            }
        }
        // Other items
        ec::itemcats::CHARGE => kinds.push(AItemKind::Charge),
        ec::itemcats::DRONE => kinds.push(AItemKind::Drone),
        ec::itemcats::FIGHTER => {
            process_fighter_kind(&mut kinds, attrs, &ec::attrs::FTR_SQ_IS_SUPPORT, AFighterKind::Support);
            process_fighter_kind(&mut kinds, attrs, &ec::attrs::FTR_SQ_IS_LIGHT, AFighterKind::Light);
            process_fighter_kind(&mut kinds, attrs, &ec::attrs::FTR_SQ_IS_HEAVY, AFighterKind::Heavy);
            process_fighter_kind(
                &mut kinds,
                attrs,
                &ec::attrs::FTR_SQ_IS_STANDUP_SUPPORT,
                AFighterKind::StandupSupport,
            );
            process_fighter_kind(
                &mut kinds,
                attrs,
                &ec::attrs::FTR_SQ_IS_STANDUP_LIGHT,
                AFighterKind::StandupLight,
            );
            process_fighter_kind(
                &mut kinds,
                attrs,
                &ec::attrs::FTR_SQ_IS_STANDUP_HEAVY,
                AFighterKind::StandupHeavy,
            );
        }
        ec::itemcats::SKILL => kinds.push(AItemKind::Skill),
        ec::itemcats::SUBSYSTEM => {
            if attrs.contains_key(&ec::attrs::SUBSYSTEM_SLOT) {
                kinds.push(AItemKind::Subsystem);
            }
        }
        _ => (),
    }
    match grp_id {
        ec::itemgrps::CHARACTER => kinds.push(AItemKind::Character),
        ec::itemgrps::EFFECT_BEACON => kinds.push(AItemKind::EffectBeacon),
        ec::itemgrps::MUTAPLASMID => kinds.push(AItemKind::Mutator),
        ec::itemgrps::SHIP_MOD => kinds.push(AItemKind::Stance),
        _ => (),
    }
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}

fn process_fighter_kind(
    kinds: &mut SmallVec<AItemKind, 1>,
    attrs: &StMap<EAttrId, AttrVal>,
    attr_id: &EAttrId,
    kind: AFighterKind,
) {
    if let Some(&val) = attrs.get(attr_id) {
        if val != OF(0.0) {
            kinds.push(AItemKind::FighterSquad(kind));
        }
    }
}
