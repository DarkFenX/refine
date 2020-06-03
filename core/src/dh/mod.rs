//! Data handling interface.
//!
//! # Assumptions about data
//! REEFAST makes several assumptions about data. It verifies them during cache generation, and if
//! those assumptions are broken, offending entries will be adjusted or removed during cache
//! generation (conversion of data into [cached types](crate::ct)).
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
//! ### Ability-to-effect mapping
//! When a player activates a fighter ability in EVE, it runs an effect mapped to it. EVE client
//! exposes no data on which fighter ability runs which effect - it is hardcoded into the client.
//! REEFAST hardcodes the map as well, thus [`dh::FighterAbil`](crate::dh::FighterAbil) without
//! corresponding entry in the map are removed, and all
//! [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil) related to them are removed too.
//!
//! ### Ability-to-effect data transfer
//! REEFAST assumes that effects which power fighter abilities are used only by those abilities and
//! nothing else. During cache generation, this assumption allows to move all the fighter ability
//! data to data structures related to effects.
//!
//! - Data defined on [`dh::FighterAbil`](crate::dh::FighterAbil) is moved to cached effects.
//! - Data defined on [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil) is moved to objects which
//!   are stored on cached items and describe per-effect properties.
//!
//! Since multiple abilities can map to the same effect, collisions are possible. In case of
//! collisions, data from colliding abilities is compared. If there are any mismatches, warnings are
//! logged, and data from the first seen entry is used.

pub use aux::{Container, Result};
pub use data::{
    Attr, Buff, BuffIM, BuffLGM, BuffLM, BuffLRSM, Effect, EffectMod, FighterAbil, Item, ItemAttr, ItemEffect,
    ItemFighterAbil, ItemGroup, ItemSkillReq, MutaAttrMod, MutaItemConv, Primitive,
};
pub use handler::DataHandler;

mod aux;
mod data;
mod handler;
