use crate::{
    misc::{DefOption, DefOptionExt},
    num::UnitInterval,
    ud::{ItemId, ProjecteeUidError, UData, UItemId},
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone, Default)]
pub struct StatCapBlcSrcKinds {
    pub default: bool,
    pub regen: DefOptionExt<StatCapBlcRegen> = DefOptionExt::Default,
    pub cap_injectors: DefOption = DefOption::Default,
    pub nosfs: DefOptionExt<StatCapBlcNosfs> = DefOptionExt::Default,
    pub consumers: DefOption = DefOption::Default,
    pub incoming_transfers: DefOption = DefOption::Default,
    pub incoming_neuts: DefOption = DefOption::Default,
}

#[derive(Copy, Clone, Default)]
pub struct StatCapBlcRegen {
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

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
