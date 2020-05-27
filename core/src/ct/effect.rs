use crate::consts::State;
use crate::defines::ReeInt;

use super::StdAttrMod;

/// Represents a dogma effect.
///
/// Effects are higher-level modification descriptors, as opposed to modifiers, which are
/// lower-level. An effect can contain any amount of modifiers under a single roof, accompanied by
/// extra effect-wide properties.
#[derive(Debug)]
pub struct Effect {
    // Effect ID.
    pub id: ReeInt,
    // Effect state dictates when the effect is activated.
    pub state: State,
    /// Defines if the effect is considered as an assistance.
    pub is_assistance: bool,
    /// Defines if the effect is offensive or not.
    pub is_offensive: bool,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub tracking_speed_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub fitting_usage_chance_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<ReeInt>,
    /// Standard attribute modifiers carried by the effect
    pub mods: Vec<StdAttrMod>,
}
impl Effect {
    /// Make a new dogma effect out of passed data.
    pub fn new(
        id: ReeInt,
        state: State,
        is_assistance: bool,
        is_offensive: bool,
        discharge_attr_id: Option<ReeInt>,
        duration_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        tracking_speed_attr_id: Option<ReeInt>,
        fitting_usage_chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
        mods: Vec<StdAttrMod>,
    ) -> Effect {
        Effect {
            id,
            state,
            is_assistance,
            is_offensive,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            tracking_speed_attr_id,
            fitting_usage_chance_attr_id,
            resist_attr_id,
            mods,
        }
    }
}
