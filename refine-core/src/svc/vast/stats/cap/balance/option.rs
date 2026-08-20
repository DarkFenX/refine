use crate::{
    DefOption, DefOptionExt, ItemId, OptionExt, UnitInterval,
    ud::{ProjecteeUidError, UData, UItemId},
};

/// Capacitor change sources which will be considered for cap balance stats.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Copy, Clone)]
pub struct StatCapBlcSrcKinds<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::src_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    regen: DefOptionExt<StatCapBlcRegen> = DefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    cap_injectors: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    nosfs: DefOptionExt<StatCapBlcNosfs<I>> = DefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    consumers: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    incoming_transfers: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    incoming_neuts: DefOption = DefOption::Default,
}
const impl<I> Default for StatCapBlcSrcKinds<I> {
    fn default() -> Self {
        Self { .. }
    }
}
impl<I> StatCapBlcSrcKinds<I> {
    /// True to have all supported sources enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    pub fn with_regen(mut self, option: OptionExt<StatCapBlcRegen>) -> Self {
        self.regen = option.into();
        self
    }
    pub fn with_cap_injectors(mut self, enabled: bool) -> Self {
        self.cap_injectors = enabled.into();
        self
    }
    pub fn with_nosfs(mut self, option: OptionExt<StatCapBlcNosfs<I>>) -> Self {
        self.nosfs = option.into();
        self
    }
    pub fn with_consumers(mut self, enabled: bool) -> Self {
        self.consumers = enabled.into();
        self
    }
    pub fn with_incoming_transfers(mut self, enabled: bool) -> Self {
        self.incoming_transfers = enabled.into();
        self
    }
    pub fn with_incoming_neuts(mut self, enabled: bool) -> Self {
        self.incoming_neuts = enabled.into();
        self
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatCapBlcRegen {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::cap_perc_default"))]
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Copy, Clone)]
pub struct StatCapBlcNosfs<I = ItemId> {
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatCapBlcNosfs<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I1> StatCapBlcSrcKinds<I1> {
    pub fn try_map_ids<I2, E, M>(self, mut item_mapper: M) -> Result<StatCapBlcSrcKinds<I2>, E>
    where
        M: FnMut(I1) -> Result<I2, E>,
    {
        Ok(StatCapBlcSrcKinds {
            default: self.default,
            regen: self.regen,
            cap_injectors: self.cap_injectors,
            nosfs: match self.nosfs {
                DefOptionExt::Default => DefOptionExt::Default,
                DefOptionExt::Disabled => DefOptionExt::Disabled,
                DefOptionExt::Enabled => DefOptionExt::Enabled,
                DefOptionExt::EnabledExtended(nosfs) => DefOptionExt::EnabledExtended(StatCapBlcNosfs {
                    projectee_item_id: match nosfs.projectee_item_id {
                        Some(projectee_item_id) => Some(item_mapper(projectee_item_id)?),
                        None => None,
                    },
                }),
            },
            consumers: self.consumers,
            incoming_transfers: self.incoming_transfers,
            incoming_neuts: self.incoming_neuts,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal-only
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) struct StatCapBlcSrcKindsInt {
    pub(crate) regen: Option<StatCapBlcRegen>,
    pub(crate) cap_injectors: bool,
    pub(crate) nosfs: Option<StatCapBlcNosfsOptionsInt>,
    pub(crate) consumers: bool,
    pub(crate) incoming_transfers: bool,
    pub(crate) incoming_neuts: bool,
}
impl StatCapBlcSrcKindsInt {
    pub(crate) fn from_pub(pub_opts: &StatCapBlcSrcKinds, u_data: &UData) -> Result<Self, ProjecteeUidError> {
        Ok(Self {
            regen: pub_opts.regen.is_enabled(pub_opts.default),
            cap_injectors: pub_opts.cap_injectors.is_enabled(pub_opts.default),
            nosfs: match pub_opts.nosfs.is_enabled(pub_opts.default) {
                Some(pub_opt) => Some(StatCapBlcNosfsOptionsInt::from_pub(pub_opt, u_data)?),
                None => None,
            },
            consumers: pub_opts.consumers.is_enabled(pub_opts.default),
            incoming_transfers: pub_opts.incoming_transfers.is_enabled(pub_opts.default),
            incoming_neuts: pub_opts.incoming_neuts.is_enabled(pub_opts.default),
        })
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct StatCapBlcNosfsOptionsInt {
    pub(crate) projectee_item_uid: Option<UItemId> = None,
}
impl StatCapBlcNosfsOptionsInt {
    fn from_pub(pub_opt: StatCapBlcNosfs, u_data: &UData) -> Result<Self, ProjecteeUidError> {
        Ok(Self {
            projectee_item_uid: match pub_opt.projectee_item_id {
                Some(projectee_item_id) => Some(u_data.get_projectee_uid(&projectee_item_id)?),
                None => None,
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use super::*;

    pub(super) fn src_default() -> bool {
        true
    }

    pub(super) fn cap_perc_default() -> UnitInterval {
        UnitInterval::from_f64_clamped(0.25)
    }
}
