use crate::{
    ed::{EAttrId, EBuffId, EItemGrpId, EItemId},
    util::Named,
};

/// EVE buff data.
pub struct EBuff {
    /// Buff ID.
    pub id: EBuffId,
    /// Defines how multiple buffs of the same type are aggregated.
    pub aggregate_mode: String,
    /// Name of the operation applied to attributes targeted by the buff.
    pub operation: String,
    /// Modifiers which apply some modification to some item directly.
    pub item_mods: Vec<EBuffIM>,
    /// Modifiers which apply some modification to location-filtered items.
    pub loc_mods: Vec<EBuffLM>,
    /// Modifiers which apply some modification to location- and group-filtered items.
    pub locgroup_mods: Vec<EBuffLGM>,
    /// Modifiers which apply some modification to location- and skill requirement-filtered items.
    pub locsrq_mods: Vec<EBuffLRSM>,
}
impl Named for EBuff {
    fn get_name() -> &'static str {
        "EBuff"
    }
}
impl std::fmt::Display for EBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}

/// Auxiliary data needed to apply an EVE buff modification directly to some item.
pub struct EBuffIM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: EAttrId,
}

/// Auxiliary data needed to apply an EVE buff modification to location-filtered items.
pub struct EBuffLM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: EAttrId,
}

/// Auxiliary data needed to apply an EVE buff modification to location- and group-filtered items.
pub struct EBuffLGM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: EAttrId,
    /// Refers an item group for a modification filter. Only items belonging to this group are
    /// eligible for the modification.
    pub group_id: EItemGrpId,
}

/// Auxiliary data needed to apply an EVE buff modification to location- and skill
/// requirement-filtered items.
pub struct EBuffLRSM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: EAttrId,
    /// Refers a skill item for a modification filter. Only items having this skill requirement will
    /// be eligible for the modification.
    pub skill_id: EItemId,
}
