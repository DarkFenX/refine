use std::error;
use std::fmt;
use std::result;

use super::data::{
    Attr, Buff, Container, Effect, FighterAbil, Item, ItemAttr, ItemEffect, ItemFighterAbil, ItemGroup,
    ItemSkillReq, MutaAttrMod, MutaItemConv,
};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait DataHandler: fmt::Debug {
    fn get_items(&self) -> Result<Container<Item>>;
    fn get_item_groups(&self) -> Result<Container<ItemGroup>>;
    fn get_attrs(&self) -> Result<Container<Attr>>;
    fn get_item_attrs(&self) -> Result<Container<ItemAttr>>;
    fn get_effects(&self) -> Result<Container<Effect>>;
    fn get_item_effects(&self) -> Result<Container<ItemEffect>>;
    fn get_fighter_abils(&self) -> Result<Container<FighterAbil>>;
    fn get_item_fighter_abils(&self) -> Result<Container<ItemFighterAbil>>;
    fn get_buffs(&self) -> Result<Container<Buff>>;
    fn get_item_skill_reqs(&self) -> Result<Container<ItemSkillReq>>;
    fn get_muta_item_convs(&self) -> Result<Container<MutaItemConv>>;
    fn get_muta_attr_mods(&self) -> Result<Container<MutaAttrMod>>;
    fn get_version(&self) -> Result<String>;
}
