use crate::defines::ReeInt;
use crate::util::Named;

/// Dogma buff data.
#[derive(Debug)]
pub struct Buff {
    /// Dogma buff ID.
    pub id: ReeInt,
    /// Defines how multiple buffs of the same type are aggregated.
    pub aggregate_mode: String,
    /// Name of the operation applied to attributes targeted by the buff.
    pub operation: String,
    /// Modifiers which apply some modification to some item directly.
    pub item_mods: Vec<BuffIM>,
    /// Modifiers which apply some modification to location-filtered items.
    pub loc_mods: Vec<BuffLM>,
    /// Modifiers which apply some modification to location- and group-filtered items.
    pub locgroup_mods: Vec<BuffLGM>,
    /// Modifiers which apply some modification to location- and skill requirement-filtered items.
    pub locsrq_mods: Vec<BuffLRSM>,
}
impl Buff {
    /// Make a new dogma buff out of passed data.
    pub fn new<T: Into<String>, U: Into<String>>(
        id: ReeInt,
        aggregate_mode: T,
        operation: U,
        item_mods: Vec<BuffIM>,
        loc_mods: Vec<BuffLM>,
        locgroup_mods: Vec<BuffLGM>,
        locsrq_mods: Vec<BuffLRSM>,
    ) -> Buff {
        Buff {
            id,
            aggregate_mode: aggregate_mode.into(),
            operation: operation.into(),
            item_mods,
            loc_mods,
            locgroup_mods,
            locsrq_mods,
        }
    }
}
impl Named for Buff {
    fn get_name() -> &'static str {
        "dh::Buff"
    }
}

/// Auxiliary data needed to apply a dogma buff modification directly to some item.
#[derive(Debug)]
pub struct BuffIM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
}
impl BuffIM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt) -> BuffIM {
        BuffIM { attr_id }
    }
}

/// Auxiliary data needed to apply a dogma buff modification to location-filtered items.
#[derive(Debug)]
pub struct BuffLM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
}
impl BuffLM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt) -> BuffLM {
        BuffLM { attr_id }
    }
}

/// Auxiliary data needed to apply a dogma buff modification to location- and group-filtered items.
#[derive(Debug)]
pub struct BuffLGM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
    /// Refers an item group for a modification filter. Only items belonging to this group are
    /// eligible for the modification.
    pub group_id: ReeInt,
}
impl BuffLGM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt, group_id: ReeInt) -> BuffLGM {
        BuffLGM { attr_id, group_id }
    }
}

/// Auxiliary data needed to apply a dogma buff modification to location- and skill
/// requirement-filtered items.
#[derive(Debug)]
pub struct BuffLRSM {
    /// Refers an attribute which is the target of the modification.
    pub attr_id: ReeInt,
    /// Refers a skill item for a modification filter. Only items having this skill requirement will
    /// be eligible for the modification.
    pub skill_id: ReeInt,
}
impl BuffLRSM {
    /// Make a new dogma buff auxiliary modifier out of passed data.
    pub fn new(attr_id: ReeInt, skill_id: ReeInt) -> BuffLRSM {
        BuffLRSM { attr_id, skill_id }
    }
}
