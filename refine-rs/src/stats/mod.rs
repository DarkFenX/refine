// Stats seem to be more complex than other commands, so worth to describe high-level logic in a
// comment.
//
// Stats options are split in two kinds: those which are simple (enabled/disabled), and those which
// can take extended options when they are enabled (like target to apply damage to). Both are stored
// in sparse format, to keep commands which store stats options small. Commands can be created using
// builder pattern, which hides all the implementation details.
//
// For some stat options of extended kind, proper backreference support complicates it further. In
// order to fail specific stat option instead of failing whole request, non-backref-capable
// container carries both variants: proper stat option and backreference resolution error.
//
// Since the same set of options can be used to fetch stats of different entities, they are resolved
// into format more convenient for stats generation process. It is dense, but it lives only during
// the resolution process itself.
//
// Regardless of stat kind, results are returned as a vector (in case of the simple kind, len is
// always 1). It is done to make the format more future-proof, during development many stats
// received options while I thought they wouldn't receive any.
//
// During processing of a single stat errors might occur. They can be split into two groups: fatal
// and non-fatal. Fatal errors fail stat for the whole set of passed options (e.g. attempt to fetch
// some stat from an item which is not loaded is fatal), while non-fatal errors fail fetch for that
// specific stat option.

pub use fit::{
    FitStats, FitStatsCmd, FitStatsCmdBr, FitStatsEnumCmdBr, FitStatsOptions, FitStatsOptionsBr, FitStatsResp,
};
pub(crate) use fit::{FitStatsCmdGen, FitStatsEnumCmd, FitStatsOptionsGen};
pub(crate) use fleet::FleetStatsOptionsGen;
pub use fleet::{FleetStats, FleetStatsCmd, FleetStatsCmdBr, FleetStatsOptions, FleetStatsOptionsBr, FleetStatsResp};
pub(crate) use item::ItemStatsOptionsGen;
pub use item::{ItemStats, ItemStatsCmd, ItemStatsCmdBr, ItemStatsOptions, ItemStatsOptionsBr, ItemStatsResp};
pub(in crate::stats) use option::StatOptionInt;
pub use option::{
    StatOptionCapBlc, StatOptionCapBlcBr, StatOptionCapSim, StatOptionCapSimBr, StatOptionEhp, StatOptionErps,
    StatOptionExt, StatOptionFitDmg, StatOptionFitDmgBr, StatOptionFitMining, StatOptionFitOutCps,
    StatOptionFitOutCpsBr, StatOptionFitOutNps, StatOptionFitOutNpsBr, StatOptionFitOutRps, StatOptionFitOutRpsBr,
    StatOptionIncomingJam, StatOptionItemDmg, StatOptionItemDmgBr, StatOptionItemMining, StatOptionItemOutCps,
    StatOptionItemOutCpsBr, StatOptionItemOutNps, StatOptionItemOutNpsBr, StatOptionItemOutRps, StatOptionItemOutRpsBr,
    StatOptionJump, StatOptionJumpBr, StatOptionMass, StatOptionRps,
};
pub(crate) use option::{
    StatOptionCapBlcGen, StatOptionCapSimGen, StatOptionFitDmgGen, StatOptionFitOutCpsGen, StatOptionFitOutNpsGen,
    StatOptionFitOutRpsGen, StatOptionItemDmgGen, StatOptionItemOutCpsGen, StatOptionItemOutNpsGen,
    StatOptionItemOutRpsGen, StatOptionJumpGen,
};
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
pub use result_details::{
    StatBrFallibleError, StatDmg, StatDmgEntry, StatDmgEntryBreacher, StatDmgEntryBreacherRaw, StatResult,
};
pub use sol::{SolStatsCmd, SolStatsCmdBr, SolStatsEnumCmdBr, SolStatsResp};
pub(crate) use sol::{SolStatsCmdGen, SolStatsEnumCmd};

pub mod err;
mod exec_shared;
mod fatal;
mod fit;
mod fleet;
mod item;
mod option;
mod result_details;
mod sol;
