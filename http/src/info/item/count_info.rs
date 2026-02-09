use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(in crate::info) struct HCountInfo {
    current: u32,
    max: u32,
    overridden: bool,
}
impl HCountInfo {
    pub(in crate::info) fn from_core_spool_cycle_count(core_spool_cycle_count: rc::SpoolCycleCountInfo) -> Self {
        Self {
            current: core_spool_cycle_count.current.into_u32(),
            max: core_spool_cycle_count.max.into_u32(),
            overridden: core_spool_cycle_count.overridden,
        }
    }
    pub(in crate::info) fn from_core_fighter_count(core_fighter_count: rc::FighterCountInfo) -> Self {
        Self {
            current: core_fighter_count.current.into_u32(),
            max: core_fighter_count.max.into_u32(),
            overridden: core_fighter_count.overridden,
        }
    }
}
