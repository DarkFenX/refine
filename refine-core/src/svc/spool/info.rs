use crate::{Count, EffectId, Spool};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ItemSpoolInfo {
    /// Spool setting used by the item.
    pub setting: Spool,
    /// Cycle counts of spoolable effects of the item.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub cycles: Vec<SpoolCycleInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct SpoolCycleInfo {
    pub effect_id: EffectId,
    /// Count of cycles at which effect reaches current spool setting.
    pub current: Count,
    /// Count of cycles at which effect reaches max spool.
    pub max: Count,
}
