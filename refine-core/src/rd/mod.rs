//! RD stands for Runtime Data.
//!
//! This module contains all the entities enriched during runtime. Runtime modifications serve 3
//! distinct purposes:
//! - Precalculating and exposing some of an entity attributes in immediately available way to save
//!   resources when they are needed. Some of the data could've been calculated on cache generation
//!   and persisted, but it makes cache handler more complex;
//! - Combining adapted and hardcoded data (as well as derived from both of those) under one roof;
//! - Remapping some IDs to arena-alike storage IDs for faster access to some entity types.

pub(crate) use data::{
    RAbil, RAttr, RAttrConsts, RAttrId, RBuff, RBuffId, RBuffModifier, RData, REffect, REffectBuff, REffectBuffScope,
    REffectCharge, REffectChargeLoc, REffectConsts, REffectEcm, REffectId, REffectLocalOpcSpec, REffectMining,
    REffectModStrength, REffectModifier, REffectNeut, REffectProjModSpec, REffectProjOpcSpec, REffectProjecteeFilter,
    REffectResist, REffectSpoolAttrs, RItem, RItemAXt, RItemCapConsumer, RItemChargeLimit, RItemContLimit,
    RItemEffectData, RItemList, RItemListId, RItemShipLimit, RMuta, RMutaAttrRange, RShipDroneLimit, RShipKind, RState,
    RcData, RcEffect, RcItem, RcMuta,
};

mod data;
