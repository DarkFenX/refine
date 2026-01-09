use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(in crate::info) struct HAdjustableCount {
    current: u32,
    max: u32,
    overridden: bool,
}
impl HAdjustableCount {
    pub(in crate::info) fn from_core_count(core_fighter_count: rc::Adjustable<rc::Count>) -> Self {
        Self {
            current: core_fighter_count.current.into_u32(),
            max: core_fighter_count.max.into_u32(),
            overridden: core_fighter_count.overridden,
        }
    }
    pub(in crate::info) fn from_core_fighter_count(core_fighter_count: rc::Adjustable<rc::FighterCount>) -> Self {
        Self {
            current: core_fighter_count.current.into_u32(),
            max: core_fighter_count.max.into_u32(),
            overridden: core_fighter_count.overridden,
        }
    }
}
