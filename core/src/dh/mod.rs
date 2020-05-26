#![warn(missing_docs)]

//! Definition of data handling interface

pub use data::{
    Attr, Buff, BuffIM, BuffLGM, BuffLM, BuffLRSM, Container, Effect, EffectMod, FighterAbil, Item, ItemAttr,
    ItemEffect, ItemFighterAbil, ItemGroup, ItemSkillReq, MutaAttrMod, MutaItemConv, Primitive,
};
pub use handler::{DataHandler, Result};

mod data;
mod handler;
