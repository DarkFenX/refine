use crate::{
    ed::{EAttrId, EEffectCatId, EEffectId, EPrimitive},
    util::{HMap, Named},
};

/// EVE dogma effect data.
pub struct EEffect {
    /// Effect ID.
    pub id: EEffectId,
    /// Refers an effect category the effect belongs to.
    pub category_id: EEffectCatId,
    /// Defines if the effect is considered as an assistance.
    pub is_assistance: bool,
    /// Defines if the effect is offensive or not.
    pub is_offensive: bool,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub tracking_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub usage_chance_attr_id: Option<EAttrId>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<EAttrId>,
    /// Modifiers of the effect.
    pub mods: Vec<EEffectMod>,
}
impl Named for EEffect {
    fn get_name() -> &'static str {
        "EEffect"
    }
}
impl std::fmt::Display for EEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}

/// EVE dogma effect modifier data.
pub struct EEffectMod {
    /// Function which the effect modifier calls to apply its modification.
    pub func: String,
    /// Arguments to the function call.
    pub args: HMap<String, EPrimitive>,
}
