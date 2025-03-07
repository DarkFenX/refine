#![feature(exact_size_is_empty)]

//! # Reefast
//! Reefast is a library built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`](crate::ed::EveDataHandler) implementation and an
//!   [`ad::AdaptedDataHandler`](crate::ad::AdaptedDataHandler) implementation to the
//!   [`Src`](crate::Src) constructor
//! - during [`Src`](crate::Src) initialization, the library attempts to load cached adapted data.
//!   If the cached data is loaded successfully, the library compares its fingerprint (data version
//!   \+ library version at the time of cache generation) and current fingerprint (version of
//!   currently provided data + current library version). If cache couldn't be loaded or
//!   fingerprints mismatch, EVE data is fetched and converted into adapted data (this process is
//!   relatively heavy on both IO and CPU), which is then fed to adapted data handler
//!   implementation.
//! - you create [`SolarSystem`](crate::SolarSystem), and manipulate it to create fits with ships
//!   and items, and fetch data and stats

pub use defs::{
    AttrVal, Count, EAbilId, EAttrId, EAttrUnitId, EBuffId, EEffectCatId, EEffectId, EItemCatId, EItemGrpId, EItemId,
    Idx, MutaRoll, OF, SkillLevel, SlotIndex, SolFitId, SolFleetId, SolItemId, VERSION,
};
pub use sol::{
    SolAddMode, SolDmgProfile, SolEffectInfo, SolEffectMode, SolModRack, SolRmMode, SolarSystem,
    info::{
        SolAttrMutationInfo, SolAutochargeInfo, SolBoosterInfo, SolCharacterInfo, SolChargeInfo, SolDroneInfo,
        SolFighterInfo, SolFitInfo, SolFleetInfo, SolFwEffectInfo, SolImplantInfo, SolItemInfo, SolItemMutationInfo,
        SolModuleInfo, SolProjEffectInfo, SolProjInfo, SolRigInfo, SolShipInfo, SolSideEffectInfo, SolSideEffectStr,
        SolSkillInfo, SolStanceInfo, SolSubsystemInfo, SolSwEffectInfo,
    },
    svc::{
        calc::{SolAffectorInfo, SolAttrVal, SolModificationInfo, SolOpInfo},
        vast::{
            SolValCapitalModFail, SolValCapitalModItemInfo, SolValChargeGroupFail, SolValChargeSizeFail,
            SolValChargeVolumeFail, SolValDroneGroupFail, SolValDroneGroupItemInfo, SolValFighterCountFail,
            SolValItemKindFail, SolValMaxGroupFail, SolValMaxGroupItemInfo, SolValModuleStateFail,
            SolValNotLoadedItemFail, SolValOption, SolValOptions, SolValResFail, SolValResItemInfo, SolValResult,
            SolValRigSizeFail, SolValRigSizeItemInfo, SolValShipLimitFail, SolValShipLimitItemInfo,
            SolValSlotCountFail, SolValSlotIndexFail, SolValSrqFail, SolValSrqSkillInfo,
        },
    },
    uad::{
        SolItemAddAttrMutation, SolItemAddMutation, SolItemAttrMutationValue, SolItemChangeAttrMutation,
        SolMinionState, SolModuleState,
    },
};
pub use src::Src;

pub mod ad;
mod adg;
mod config;
mod defs;
pub mod ec;
pub mod ed;
pub mod err;
mod sol;
mod src;
pub mod util;
