use crate::{
    ad::{AFighterKind, AItemEffectData, AItemKind},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, OF},
    ec,
    util::StMap,
};

/// Holds extra item-specific data.
///
/// It is derived from data normally available on item and other entities, but is calculated on
/// cache generation time for optimization purposes.
#[derive(Clone)]
pub struct AItemExtras {
    /// Item type.
    pub kind: Option<AItemKind>,
    /// Unmodified item volume.
    pub volume: Option<AttrVal>,
}
impl AItemExtras {
    pub(crate) fn new_with_data(
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
    ) -> Self {
        let mut extras = Self {
            kind: None,
            volume: None,
        };
        extras.update(grp_id, cat_id, attrs, effects);
        extras
    }
    pub(crate) fn update(
        &mut self,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
    ) {
        self.kind = get_item_kind(grp_id, cat_id, attrs, effects);
        self.volume = attrs.get(&ec::attrs::VOLUME).map(|v| *v);
    }
}

fn get_item_kind(
    grp_id: EItemGrpId,
    cat_id: EItemCatId,
    attrs: &StMap<EAttrId, AttrVal>,
    effects: &StMap<EEffectId, AItemEffectData>,
) -> Option<AItemKind> {
    let mut kinds = Vec::new();
    if cat_id == ec::itemcats::IMPLANT && attrs.contains_key(&ec::attrs::BOOSTERNESS) {
        kinds.push(AItemKind::Booster);
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
        kinds.push(AItemKind::Implant);
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
    if cat_id == ec::itemcats::SUBSYSTEM && effects.contains_key(&ec::effects::SUBSYSTEM) {
        kinds.push(AItemKind::Subsystem);
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
