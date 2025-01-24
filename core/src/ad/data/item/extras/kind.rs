use crate::{
    ad::AItemEffectData,
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, SlotIndex, OF},
    ec,
    util::StMap,
};

/// Contains adapted item types.
#[derive(Copy, Clone)]
pub enum AItemKind {
    Booster(SlotIndex),
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad(AFighterKind),
    Implant(SlotIndex),
    ModHigh,
    ModLow,
    ModMid,
    Mutator,
    Rig,
    Ship,
    Skill,
    Stance,
    Subsystem(SlotIndex),
}

/// Contains adapted fighter squad types.
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
) -> Option<AItemKind> {
    let mut kinds = Vec::new();
    if cat_id == ec::itemcats::IMPLANT && attrs.contains_key(&ec::attrs::BOOSTERNESS) {
        kinds.push(AItemKind::Booster(
            attrs.get(&ec::attrs::BOOSTERNESS).unwrap().round() as SlotIndex
        ));
    };
    if grp_id == ec::itemgrps::CHARACTER {
        kinds.push(AItemKind::Character);
    };
    if cat_id == ec::itemcats::CHARGE {
        kinds.push(AItemKind::Charge);
    };
    if cat_id == ec::itemcats::DRONE {
        kinds.push(AItemKind::Drone);
    };
    if grp_id == ec::itemgrps::EFFECT_BEACON {
        kinds.push(AItemKind::EffectBeacon);
    };
    if cat_id == ec::itemcats::FIGHTER {
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
    };
    if cat_id == ec::itemcats::IMPLANT && attrs.contains_key(&ec::attrs::IMPLANTNESS) {
        kinds.push(AItemKind::Implant(
            attrs.get(&ec::attrs::IMPLANTNESS).unwrap().round() as SlotIndex
        ));
    };
    if cat_id == ec::itemcats::MODULE && effects.contains_key(&ec::effects::HI_POWER) {
        kinds.push(AItemKind::ModHigh);
    };
    if cat_id == ec::itemcats::MODULE && effects.contains_key(&ec::effects::LO_POWER) {
        kinds.push(AItemKind::ModLow);
    };
    if cat_id == ec::itemcats::MODULE && effects.contains_key(&ec::effects::MED_POWER) {
        kinds.push(AItemKind::ModMid);
    };
    if cat_id == ec::itemcats::MODULE && effects.contains_key(&ec::effects::RIG_SLOT) {
        kinds.push(AItemKind::Rig);
    };
    if grp_id == ec::itemgrps::MUTAPLASMID {
        kinds.push(AItemKind::Mutator);
    };
    if cat_id == ec::itemcats::SHIP {
        kinds.push(AItemKind::Ship);
    };
    if cat_id == ec::itemcats::SKILL {
        kinds.push(AItemKind::Skill);
    };
    if grp_id == ec::itemgrps::SHIP_MOD {
        kinds.push(AItemKind::Stance);
    };
    if cat_id == ec::itemcats::SUBSYSTEM && attrs.contains_key(&ec::attrs::SUBSYSTEM_SLOT) {
        kinds.push(AItemKind::Subsystem(
            attrs.get(&ec::attrs::SUBSYSTEM_SLOT).unwrap().round() as SlotIndex,
        ));
    };
    match kinds.len() {
        1 => Some(kinds.pop().unwrap()),
        _ => None,
    }
}

fn process_fighter_kind(
    kinds: &mut Vec<AItemKind>,
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
