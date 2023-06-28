use crate::{
    defs::{AttrId, BuffId, ItemGrpId, ItemId},
    util::Named,
};

/// EVE buff data.
#[derive(Debug)]
pub struct EBuff {
    /// Buff ID.
    pub id: BuffId,
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
impl EBuff {
    /// Make a new EVE buff out of passed data.
    pub fn new(
        id: BuffId,
        aggregate_mode: String,
        operation: String,
        item_mods: Vec<EBuffIM>,
        loc_mods: Vec<EBuffLM>,
        locgroup_mods: Vec<EBuffLGM>,
        locsrq_mods: Vec<EBuffLRSM>,
    ) -> Self {
        Self {
            id,
            aggregate_mode,
            operation,
            item_mods,
            loc_mods,
            locgroup_mods,
            locsrq_mods,
        }
    }
}
impl Named for EBuff {
    fn get_name() -> &'static str {
        "EBuff"
    }
}

/// Auxiliary data needed to apply an EVE buff modification directly to some item.
#[derive(Debug)]
pub struct EBuffIM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: AttrId,
}
impl EBuffIM {
    /// Make a new EVE buff auxiliary modifier out of passed data.
    pub fn new(attr_id: AttrId) -> Self {
        Self { attr_id }
    }
}

/// Auxiliary data needed to apply an EVE buff modification to location-filtered items.
#[derive(Debug)]
pub struct EBuffLM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: AttrId,
}
impl EBuffLM {
    /// Make a new EVE buff auxiliary modifier out of passed data.
    pub fn new(attr_id: AttrId) -> Self {
        Self { attr_id }
    }
}

/// Auxiliary data needed to apply an EVE buff modification to location- and group-filtered items.
#[derive(Debug)]
pub struct EBuffLGM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: AttrId,
    /// Refers an item group for a modification filter. Only items belonging to this group are
    /// eligible for the modification.
    pub group_id: ItemGrpId,
}
impl EBuffLGM {
    /// Make a new EVE buff auxiliary modifier out of passed data.
    pub fn new(attr_id: AttrId, group_id: ItemGrpId) -> Self {
        Self { attr_id, group_id }
    }
}

/// Auxiliary data needed to apply an EVE buff modification to location- and skill
/// requirement-filtered items.
#[derive(Debug)]
pub struct EBuffLRSM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: AttrId,
    /// Refers a skill item for a modification filter. Only items having this skill requirement will
    /// be eligible for the modification.
    pub skill_id: ItemId,
}
impl EBuffLRSM {
    /// Make a new EVE buff auxiliary modifier out of passed data.
    pub fn new(attr_id: AttrId, skill_id: ItemId) -> Self {
        Self { attr_id, skill_id }
    }
}
