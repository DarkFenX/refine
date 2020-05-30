use crate::{consts::State, defines::ReeInt};

use super::AttrMod;

/// Represents a dogma effect.
///
/// Effects are higher-level modification descriptors, as opposed to modifiers, which are
/// lower-level. An effect can contain any amount of modifiers under a single roof, accompanied by
/// extra effect-wide properties.
#[derive(Debug)]
pub struct Effect {
    /// Effect ID.
    pub id: ReeInt,
    /// Effect state dictates which state of parent item is needed for the effect to run.
    pub state: State,
    /// Does the effect need a target or not to run.
    pub tgtd: bool,
    /// Defines if the effect is considered as an assistance.
    pub is_assist: bool,
    /// Defines if the effect is offensive or not.
    pub is_offense: bool,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub track_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub chance_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<ReeInt>,
    /// Standard attribute modifiers carried by the effect
    pub mods: Vec<AttrMod>,
    /// Refers effects this effect stops.
    pub stop_ids: Vec<ReeInt>,
}
impl Effect {
    /// Make a new dogma effect out of passed data.
    pub fn new(
        id: ReeInt,
        state: State,
        tgtd: bool,
        is_assist: bool,
        is_offense: bool,
        discharge_attr_id: Option<ReeInt>,
        duration_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        track_attr_id: Option<ReeInt>,
        chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
        mods: Vec<AttrMod>,
        stop_ids: Vec<ReeInt>,
    ) -> Effect {
        Effect {
            id,
            state,
            tgtd,
            is_assist,
            is_offense,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            track_attr_id,
            chance_attr_id,
            resist_attr_id,
            mods,
            stop_ids,
        }
    }
}
