//! EVE data handler and data types.
//!
//! Please be aware that neither handler interface nor data types should be considered as stable.
//! Whenever CCP significantly change the EVE data format, the interface has to change as well.
//!
//! # Assumptions about data
//! Refine verifies data integrity and makes several assumptions about it. If those assumptions are
//! broken, offending entries will be adjusted or ignored during conversion of the data into
//! [adapted data](crate::ad).
//!
//! ### Primary keys
//! Almost every data entry provided by a [`ed::EveDataHandler`](EveDataHandler) implementation has
//! a private PK getter defined. For every vector there can be maximum one entry with the same PK.
//! When there are multiple entries with the same PK, only first seen entry is kept, with the rest
//! being completely ignored.
//!
//! ### Item's default dogma effect
//! Every item can have a maximum of one default dogma effect. For any given item,
//! [`ed::EItemEffect`](EItemEffect) which is marked as default will be marked as non-default past
//! first seen entry.
//!
//! ### Ability-to-effect data transfer
//! Refine assumes that dogma effects which power fighter abilities are used only by those
//! abilities and nothing else. During data adaptation, this assumption allows to move all the
//! fighter ability data to data structures related to dogma effects.
//!
//! - Data defined on [`ed::EFighterAbil`](EAbil) is moved to [`ad::AEffect`](crate::ad::AEffect).
//! - Data defined on [`ed::EItemFighterAbil`](EItemAbil) is moved to
//!   [`ad::AItemEffectData`](crate::ad::AItemEffectData), which describe effect properties specific
//!   to parent [`ad::AItem`](crate::ad::AItem).
//!
//! Since multiple abilities can map to the same dogma effect, collisions are possible. In case of
//! collisions, data from colliding abilities is compared. If there are any mismatches, warnings are
//! logged, and data is not transferred to a dogma effect.

pub use data::{
    EAbil, EAbilId, EAttr, EAttrId, EAttrUnitId, EBuff, EBuffIM, EBuffId, EBuffLGM, EBuffLM, EBuffLRSM, EData,
    EDataCont, EEffect, EEffectCatId, EEffectId, EEffectMod, EEffectModArg, EFloat, EInt, EItem, EItemAbil, EItemAttr,
    EItemCatId, EItemEffect, EItemGroup, EItemGrpId, EItemId, EItemList, EItemListId, EItemSkillReq, EItemSpaceComp,
    EItemSpaceCompBuffData, EItemSpaceCompBuffEntry, EMutaAttrMod, EMutaItemConv, EPrimitive,
};
pub use handler::EveDataHandler;
pub use result::EResult;

mod consts;
mod data;
mod handler;
mod result;
