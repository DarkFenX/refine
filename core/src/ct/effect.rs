use crate::consts::EveEffectCategory;
use crate::defines::ReeInt;

pub struct Effect {
    pub id: ReeInt,
    pub category_id: EveEffectCategory,
    pub is_offensive: bool,
    pub is_assistance: bool,
    pub duration_attr_id: Option<ReeInt>,
    pub discharge_attr_id: Option<ReeInt>,
    pub range_attr_id: Option<ReeInt>,
    pub falloff_attr_id: Option<ReeInt>,
    pub tracking_speed_attr_id: Option<ReeInt>,
    pub fitting_usage_chance_attr_id: Option<ReeInt>,
    pub resist_attr_id: Option<ReeInt>,
}

impl Effect {
    pub fn new(
        id: ReeInt,
        category_id: EveEffectCategory,
        is_offensive: bool,
        is_assistance: bool,
        duration_attr_id: Option<ReeInt>,
        discharge_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        tracking_speed_attr_id: Option<ReeInt>,
        fitting_usage_chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
    ) -> Effect {
        Effect {
            id,
            category_id,
            is_offensive,
            is_assistance,
            duration_attr_id,
            discharge_attr_id,
            range_attr_id,
            falloff_attr_id,
            tracking_speed_attr_id,
            fitting_usage_chance_attr_id,
            resist_attr_id,
        }
    }
}
