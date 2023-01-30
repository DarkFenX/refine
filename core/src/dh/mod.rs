//! Data handling interface.
//!
//! Data handlers provide a way for the library to fetch the data it needs from external sources.
//!
//! # Assumptions about data
//! REEFAST verifies data integrity and makes several assumptions about data. If those assumptions
//! are broken, offending entries will be adjusted or removed during cache generation (conversion of
//! data into [cacheable types](crate::ct)).
//!
//! ### Primary keys
//! Almost every data entry provided by a `DataHandler` implementation has a private PK getter
//! defined. For every vector there can be maximum one entry with the same PK. When there are
//! multiple entries with the same PK, only first seen entry is kept, with the rest getting removed
//! altogether.
//!
//! ### Item's default effect
//! Every item can have a maximum of one default effect. For any given item,
//! [`dh::ItemEffect`](crate::dh::ItemEffect) which is marked as default will be marked as
//! non-default past first seen entry.
//!
//! ### Ability-to-effect data transfer
//! REEFAST assumes that effects which power fighter abilities are used only by those abilities and
//! nothing else. During cache generation, this assumption allows to move all the fighter ability
//! data to data structures related to effects.
//!
//! - Data defined on [`dh::FighterAbil`](crate::dh::FighterAbil) is moved to
//!   [`ct::Effect`](crate::ct::Effect).
//! - Data defined on [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil) is moved to
//!   [`ct::ItemEffData`](crate::ct::ItemEffData), which describe effect properties specific to
//!   parent [`ct::Item`](crate::ct::Item).
//!
//! Since multiple abilities can map to the same effect, collisions are possible. In case of
//! collisions, data from colliding abilities is compared. If there are any mismatches, warnings are
//! logged, and data is not transferred to an effect.

pub use cont::Container;
pub use data::{
    Attr, Buff, BuffIM, BuffLGM, BuffLM, BuffLRSM, Effect, EffectMod, FighterAbil, Item, ItemAttr, ItemEffect,
    ItemFighterAbil, ItemGroup, ItemSkillReq, MutaAttrMod, MutaItemConv, Primitive,
};
pub use handler::DataHandler;
pub use result::Result;

mod cont;
mod data;
mod handler;
mod result;
