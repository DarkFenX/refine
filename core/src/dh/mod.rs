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
//! We have to hardcode the map as well, thus [`dh::FighterAbil`](crate::dh::FighterAbil) without
//! corresponding entry in the map are removed, and all
//! [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil) related to them are removed too.
//!
//! Also, for every item's [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil), there has to be an
//! [`dh::ItemEffect`](crate::dh::ItemEffect) this ability points to, otherwise the ability for the
//! item will be removed.
//!
//! ### Multi-run effects
//! Another assumption REEFAST makes is that any item can have only one instance of its effect
//! running at any given time. So far, the only way to go around this assumption would be having
//! several [`dh::ItemFighterAbil`](crate::dh::ItemFighterAbil) which point to different abilities,
//! which map to the same effect. To eliminate that, excessive entries are removed.

pub use aux::{Container, Result};
pub use data::{
    Attr, Buff, BuffIM, BuffLGM, BuffLM, BuffLRSM, Effect, EffectMod, FighterAbil, Item, ItemAttr, ItemEffect,
    ItemFighterAbil, ItemGroup, ItemSkillReq, MutaAttrMod, MutaItemConv, Primitive,
};
pub use handler::DataHandler;

mod aux;
mod data;
mod handler;
