use crate::{
    ad::{AAttrId, AEffectBuffInfo, AEffectCatId, AEffectId, AEffectModifier, AEffectXt, AState},
    nd,
    util::Named,
};

/// Represents an adapted effect.
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
    pub is_assist: bool = false,
    /// Defines if the effect is offensive or not.
    pub is_offense: bool = false,
    /// Defines if the effect can be used in hisec.
    pub hisec: Option<bool> = None,
    /// Defines if the effect can be used in lowsec.
    pub lowsec: Option<bool> = None,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub track_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub chance_attr_id: Option<AAttrId> = None,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<AAttrId> = None,
    /// Attribute modifiers carried by the effect
    pub mods: Vec<AEffectModifier> = Vec::new(),
    /// Refers effects this effect stops on target.
    pub stop_ids: Vec<AEffectId> = Vec::new(),
    /// Buff carried by the effect.
    pub buff: Option<AEffectBuffInfo> = None,
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

/// Adapted effect with extra data added to it during runtime.
pub struct AEffectRt {
    /// Adapted effect.
    pub ae: AEffect,
    /// Hardcoded data for the effect.
    pub(crate) hc: nd::NEffectHc,
    /// Extra data, which is generated using adapted data and hardcoded data.
    pub(crate) xt: AEffectXt,
}
impl AEffectRt {
    /// Construct new adapted effect with runtime data.
    pub fn new(a_effect: AEffect) -> Self {
        let n_effect = nd::N_EFFECT_MAP.get(&a_effect.id);
        let xt = AEffectXt {
            proj_a_attr_ids: n_effect
                .and_then(|v| v.xt_get_proj_attrs)
                .map(|get_proj_attrs| get_proj_attrs(&a_effect))
                .unwrap_or_default(),
        };
        Self {
            ae: a_effect,
            hc: n_effect.map(|n_effect| n_effect.hc).unwrap_or_default(),
            xt,
        }
    }
}
impl Named for AEffectRt {
    fn get_name() -> &'static str {
        "AEffectRt"
    }
}
impl std::fmt::Display for AEffectRt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.ae.id)
    }
}
