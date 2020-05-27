//! Data handling interface

pub use aux::{Container, Result};
pub use data::{
    Attr, Buff, BuffIM, BuffLGM, BuffLM, BuffLRSM, Effect, EffectMod, FighterAbil, Item, ItemAttr, ItemEffect,
    ItemFighterAbil, ItemGroup, ItemSkillReq, MutaAttrMod, MutaItemConv, Primitive,
};
pub use handler::DataHandler;

mod aux;
mod data;
mod handler;
