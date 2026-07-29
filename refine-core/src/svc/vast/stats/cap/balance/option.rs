use crate::{
    DefOption, DefOptionExt, ItemId, UnitInterval,
    ud::{ProjecteeUidError, UData, UItemId},
};

/// Capacitor change sources which will be considered for cap balance stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatCapBlcSrcKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::src_default"))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub regen: DefOptionExt<StatCapBlcRegen> = DefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cap_injectors: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub nosfs: DefOptionExt<StatCapBlcNosfs> = DefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub consumers: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub incoming_transfers: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub incoming_neuts: DefOption = DefOption::Default,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatCapBlcRegen {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::cap_perc_default"))]
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatCapBlcNosfs {
    pub projectee_item_id: Option<ItemId> = None,
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
