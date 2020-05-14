use crate::consts::EveEffectCategory;
use crate::defines::Id;

pub struct Effect {
    pub id: Id,
    pub category_id: EveEffectCategory,
    pub is_offensive: bool,
    pub is_assistance: bool,
    pub duration_attr_id: Option<Id>,
    pub discharge_attr_id: Option<Id>,
    pub range_attr_id: Option<Id>,
    pub falloff_attr_id: Option<Id>,
    pub tracking_speed_attr_id: Option<Id>,
    pub fitting_usage_chance_attr_id: Option<Id>,
    pub resist_attr_id: Option<Id>,
}

impl Effect {
    pub fn new(
        id: Id,
        category_id: EveEffectCategory,
        is_offensive: bool,
        is_assistance: bool,
        duration_attr_id: Option<Id>,
        discharge_attr_id: Option<Id>,
        range_attr_id: Option<Id>,
        falloff_attr_id: Option<Id>,
        tracking_speed_attr_id: Option<Id>,
        fitting_usage_chance_attr_id: Option<Id>,
        resist_attr_id: Option<Id>,
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
