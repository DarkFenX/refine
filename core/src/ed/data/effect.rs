use crate::{
    ed::{EAttrId, EEffectCatId, EEffectId, EPrimitive},
    util::{Named, RMap},
};

pub struct EEffect {
    pub id: EEffectId,
    pub category_id: EEffectCatId,
    pub is_assistance: bool,
    pub is_offensive: bool,
    pub discharge_attr_id: Option<EAttrId>,
    pub duration_attr_id: Option<EAttrId>,
    pub range_attr_id: Option<EAttrId>,
    pub falloff_attr_id: Option<EAttrId>,
    pub tracking_attr_id: Option<EAttrId>,
    pub usage_chance_attr_id: Option<EAttrId>,
    pub resist_attr_id: Option<EAttrId>,
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

pub struct EEffectMod {
    pub func: String,
    pub args: RMap<String, EPrimitive>,
}
