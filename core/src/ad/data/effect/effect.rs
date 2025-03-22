use crate::{
    ad::{
        AAttrId, AEffectBuffInfo, AEffectCatId, AEffectChargeInfo, AEffectId, AEffectModBuildStatus, AEffectModifier,
        AState,
    },
    util::Named,
};

/// Represents an adapted dogma effect.
///
/// Effects are higher-level modification descriptors, as opposed to modifiers, which are
/// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
/// extra effect-wide properties.
pub struct AEffect {
    /// Effect ID.
    pub id: AEffectId,
    /// Effect category ID, part of definition how effect is applied.
    pub category: AEffectCatId,
    /// Effect state dictates which state of parent item is needed for the effect to run.
    pub state: AState,
    /// Defines if the effect is considered as an assistance.
    pub is_assist: bool,
    /// Defines if the effect is offensive or not.
    pub is_offense: bool,
    /// Defines if the effect can be used in hisec.
    pub hisec: Option<bool>,
    /// Defines if the effect can be used in lowsec.
    pub lowsec: Option<bool>,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub track_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub chance_attr_id: Option<AAttrId>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<AAttrId>,
    /// Modifier build status.
    pub mod_build_status: AEffectModBuildStatus,
    /// Attribute modifiers carried by the effect
    pub mods: Vec<AEffectModifier>,
    /// Refers effects this effect stops on target.
    pub stop_ids: Vec<AEffectId>,
    /// Buff carried by the effect.
    pub buff: Option<AEffectBuffInfo>,
    /// Charge used by the effect.
    pub charge: Option<AEffectChargeInfo>,
}
impl Named for AEffect {
    fn get_name() -> &'static str {
        "AEffect"
    }
}
impl std::fmt::Display for AEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
