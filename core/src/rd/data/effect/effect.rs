use crate::{
    ad,
    rd::{REffectBuffInfo, REffectKey},
    util::Named,
};

// Represents an effect.
//
// Effects are higher-level modification descriptors, as opposed to modifiers, which are
// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
// extra effect-wide properties.
pub(crate) struct REffect {
    a_effect: ad::AEffect,
    stopped_effect_keys: Vec<REffectKey> = Vec::new(),
    buff_info: Option<REffectBuffInfo> = None,
}
impl REffect {
    pub(crate) fn new(a_effect: ad::AEffect) -> Self {
        Self { a_effect, .. }
    }
    // Effect category ID, part of definition how effect is applied.
    pub(crate) fn get_category(&self) -> ad::AEffectCatId {
        self.a_effect.category
    }
    // Effect state dictates which state of parent item is needed for the effect to run.
    pub(crate) fn get_state(&self) -> ad::AState {
        self.a_effect.state
    }
    // Defines if the effect is considered as an assistance.
    pub(crate) fn is_assist(&self) -> bool {
        self.a_effect.is_assist
    }
    // Defines if the effect is offensive or not.
    pub(crate) fn is_offense(&self) -> bool {
        self.a_effect.is_offense
    }
    // Defines if the effect can be used in hisec.
    pub(crate) fn is_usable_in_hisec(&self) -> Option<bool> {
        self.a_effect.hisec
    }
    // Defines if the effect can be used in lowsec.
    pub(crate) fn is_usable_in_lowsec(&self) -> Option<bool> {
        self.a_effect.lowsec
    }
    // Refers an attribute value which defines capacitor cost to run the effect.
    pub(crate) fn get_discharge_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.discharge_attr_id
    }
    // Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub(crate) fn get_duration_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.duration_attr_id
    }
    // Refers an attribute value which defines optimal range of the effect in meters.
    pub(crate) fn get_range_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.range_attr_id
    }
    // Refers an attribute value which defines falloff range of the effect in meters.
    pub(crate) fn get_falloff_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.falloff_attr_id
    }
    // Refers an attribute value which defines tracking speed of the effect.
    pub(crate) fn get_track_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.track_attr_id
    }
    // Refers an attribute value which defines chance of the effect to run when its parent item is
    // fitted.
    pub(crate) fn get_chance_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.chance_attr_id
    }
    // Refers an attribute value which defines resistance strength to the effect.
    pub(crate) fn get_resist_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_effect.resist_attr_id
    }
    // Attribute modifiers carried by the effect.
    pub(crate) fn get_mods(&self) -> &Vec<ad::AEffectModifier> {
        &self.a_effect.mods
    }
    // Refers effects this effect stops on target.
    pub(crate) fn get_stopped_effect_keys(&self) -> &Vec<REffectKey> {
        &self.stopped_effect_keys
    }
    // Buff carried by the effect.
    pub(crate) fn get_buff_info(&self) -> Option<&REffectBuffInfo> {
        self.buff_info.as_ref()
    }
}
impl Named for REffect {
    fn get_name() -> &'static str {
        "REffect"
    }
}
