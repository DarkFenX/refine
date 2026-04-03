use crate::{
    num::UnitInterval,
    ud::{ItemId, ProjecteeUidError, UData, UItemId},
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapBlcSrcKinds {
    pub regen: StatCapBlcRegen,
    pub cap_injectors: bool,
    pub nosfs: StatCapBlcNosfs,
    pub consumers: bool,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapBlcSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: StatCapBlcRegen::Enabled(StatCapBlcRegenOptions { .. }),
            cap_injectors: true,
            nosfs: StatCapBlcNosfs::Enabled(StatCapBlcNosfsOptions { .. }),
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: StatCapBlcRegen::Disabled,
            cap_injectors: false,
            nosfs: StatCapBlcNosfs::Disabled,
            consumers: false,
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum StatCapBlcRegen {
    Enabled(StatCapBlcRegenOptions),
    Disabled,
}

#[derive(Copy, Clone)]
pub struct StatCapBlcRegenOptions {
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

#[derive(Copy, Clone)]
pub enum StatCapBlcNosfs {
    Enabled(StatCapBlcNosfsOptions),
    Disabled,
}

#[derive(Copy, Clone)]
pub struct StatCapBlcNosfsOptions {
    pub projectee_item_id: Option<ItemId> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal-only
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) struct StatCapBlcSrcKindsInt {
    pub(crate) regen: StatCapBlcRegen,
    pub(crate) cap_injectors: bool,
    pub(crate) nosfs: StatCapBlcNosfsInt,
    pub(crate) consumers: bool,
    pub(crate) incoming_transfers: bool,
    pub(crate) incoming_neuts: bool,
}
impl StatCapBlcSrcKindsInt {
    pub(crate) fn from_pub(pub_opts: &StatCapBlcSrcKinds, u_data: &UData) -> Result<Self, ProjecteeUidError> {
        Ok(Self {
            regen: pub_opts.regen,
            cap_injectors: pub_opts.cap_injectors,
            nosfs: StatCapBlcNosfsInt::from_pub(&pub_opts.nosfs, u_data)?,
            consumers: pub_opts.consumers,
            incoming_transfers: pub_opts.incoming_transfers,
            incoming_neuts: pub_opts.incoming_neuts,
        })
    }
}

#[derive(Copy, Clone)]
pub(crate) enum StatCapBlcNosfsInt {
    Enabled(StatCapBlcNosfsOptionsInt),
    Disabled,
}
impl StatCapBlcNosfsInt {
    fn from_pub(pub_opts: &StatCapBlcNosfs, u_data: &UData) -> Result<Self, ProjecteeUidError> {
        Ok(match pub_opts {
            StatCapBlcNosfs::Enabled(options) => Self::Enabled(StatCapBlcNosfsOptionsInt::from_pub(options, u_data)?),
            StatCapBlcNosfs::Disabled => Self::Disabled,
        })
    }
}

#[derive(Copy, Clone)]
pub(crate) struct StatCapBlcNosfsOptionsInt {
    pub(crate) projectee_item_uid: Option<UItemId> = None,
}
impl StatCapBlcNosfsOptionsInt {
    fn from_pub(pub_opts: &StatCapBlcNosfsOptions, u_data: &UData) -> Result<Self, ProjecteeUidError> {
        Ok(Self {
            projectee_item_uid: match &pub_opts.projectee_item_id {
                Some(projectee_item_id) => Some(u_data.get_projectee_uid(projectee_item_id)?),
                None => None,
            },
        })
    }
}
