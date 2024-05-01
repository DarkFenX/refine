use crate::{
    ad::{AEffectBuffInfo, AEffectModifier, AModBuildStatus, AState, ATgtMode},
    defs::{EAttrId, EEffectCatId, EEffectId},
    util::Named,
};

/// Represents an adapted dogma effect.
///
/// Effects are higher-level modification descriptors, as opposed to modifiers, which are
/// lower-level. An effect can contain any amount of modifiers under a single roof, accompanied by
/// extra effect-wide properties.
pub struct AEffect {
    /// Effect ID.
    pub id: EEffectId,
    /// Effect category ID, part of definition how effect is applied.
    pub category: EEffectCatId,
    /// Effect state dictates which state of parent item is needed for the effect to run.
    pub state: AState,
    /// Defines what kind of target you need to run the effect.
    pub tgt_mode: Option<ATgtMode>,
    /// Defines if the effect is considered as an assistance.
    pub is_assist: bool,
    /// Defines if the effect is offensive or not.
    pub is_offense: bool,
    /// Defines if the effect can be used in hisec.
    pub hisec: Option<bool>,
    /// Defines if the effect can be used in lowsec.
    pub lowsec: Option<bool>,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub track_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub chance_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<EAttrId>,
    /// Modifier build status.
    pub mod_build_status: AModBuildStatus,
    /// Attribute modifiers carried by the effect
    pub mods: Vec<AEffectModifier>,
    /// Refers effects this effect stops on target.
    pub stop_ids: Vec<EEffectId>,
    /// Buff carried by the effect.
    pub buff: Option<AEffectBuffInfo>,
}
impl AEffect {
    /// Make a new adapted dogma effect out of passed data.
    pub(crate) fn new(
        id: EEffectId,
        category: EEffectCatId,
        state: AState,
        tgt_mode: Option<ATgtMode>,
        is_assist: bool,
        is_offense: bool,
        hisec: Option<bool>,
        lowsec: Option<bool>,
        discharge_attr_id: Option<EAttrId>,
        duration_attr_id: Option<EAttrId>,
        range_attr_id: Option<EAttrId>,
        falloff_attr_id: Option<EAttrId>,
        track_attr_id: Option<EAttrId>,
        chance_attr_id: Option<EAttrId>,
        resist_attr_id: Option<EAttrId>,
        mod_build_status: AModBuildStatus,
        mods: Vec<AEffectModifier>,
        stop_ids: Vec<EEffectId>,
        buff: Option<AEffectBuffInfo>,
    ) -> Self {
        Self {
            id,
            category,
            state,
            tgt_mode,
            is_assist,
            is_offense,
            hisec,
            lowsec,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            track_attr_id,
            chance_attr_id,
            resist_attr_id,
            mod_build_status,
            mods,
            stop_ids,
            buff,
        }
    }
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
