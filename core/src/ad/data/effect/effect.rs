use crate::{
    ad::{AAttrId, AEffectBuffInfo, AEffectCatId, AEffectId, AEffectModifier, AEffectXt, AState},
    nd,
    util::Named,
};

pub struct AEffect {
    pub id: AEffectId,
    pub category: AEffectCatId,
    pub state: AState,
    pub is_assist: bool = false,
    pub is_offense: bool = false,
    pub hisec: Option<bool> = None,
    pub lowsec: Option<bool> = None,
    pub discharge_attr_id: Option<AAttrId> = None,
    pub duration_attr_id: Option<AAttrId> = None,
    pub range_attr_id: Option<AAttrId> = None,
    pub falloff_attr_id: Option<AAttrId> = None,
    pub track_attr_id: Option<AAttrId> = None,
    pub chance_attr_id: Option<AAttrId> = None,
    pub resist_attr_id: Option<AAttrId> = None,
    pub mods: Vec<AEffectModifier> = Vec::new(),
    pub stoped_effect_ids: Vec<AEffectId> = Vec::new(),
    pub buff_info: Option<AEffectBuffInfo> = None,
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
            is_active: a_effect.state >= AState::Active && a_effect.duration_attr_id.is_some(),
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
