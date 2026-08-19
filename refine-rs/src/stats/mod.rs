pub(crate) use cmd_opts::{StatDefOption, StatDefOptionExt};
pub use cmd_opts::{
    StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmg,
    StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam,
    StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps,
    StatOptionJump, StatOptionMass, StatOptionRps,
};
use fatal::StatErrorFatality;
pub use fit::{FitStats, GetFitStatsCmd};
pub use fleet::{FleetStats, GetFleetStatsCmd};
pub use item::{GetItemStatsCmd, ItemStats};
pub use rc::{
    CtlAffectors as StatAffectors,
    stats::{
        StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatCritOptions, StatDmgItemKinds, StatEhp, StatEhpLayer,
        StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer, StatInJam, StatItemChargeOptions,
        StatItemStateOptions, StatJump, StatJumpConduit, StatJumpPortal, StatJumpRange, StatJumpSelf, StatMining,
        StatMiningEntry, StatMiningItemKinds, StatMiningResourceKind, StatNeutItemKinds, StatOutRepItemKinds,
        StatOutReps, StatResists, StatResistsLayer, StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen,
        StatSensors, StatSensorsKind, StatSlot, StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim,
    },
};
pub use result_details::{StatDmg, StatDmgEntry, StatDmgEntryBreacher, StatDmgEntryBreacherRaw, StatResult};

mod cmd_core;
mod cmd_opts;
pub mod err;
mod fatal;
mod fit;
mod fleet;
mod item;
mod result_details;
