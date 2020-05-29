use std::fmt;

use super::{
    aux::{Container, Result},
    data::{
        Attr, Buff, Effect, FighterAbil, Item, ItemAttr, ItemEffect, ItemFighterAbil, ItemGroup, ItemSkillReq,
        MutaAttrMod, MutaItemConv,
    },
};

/// Data handler interface definition.
///
/// Please be aware that this interface is not expected to be stable. Whenever
/// CCP significantly change the EVE data format, the interface has to change as
/// well.
///
/// All the methods required by this trait should return an error only when it
/// is impossible to fetch the data altogether. In case of a less impactful
/// error (such as inability to deserialize one specific item within a big array
/// of data), the error should be recorded as a meaningful message and stored in
/// [`Container::errors`](self::Container::errors).
pub trait DataHandler: fmt::Debug {
    /// Get item types.
    fn get_items(&self) -> Result<Container<Item>>;
    /// Get item groups.
    fn get_item_groups(&self) -> Result<Container<ItemGroup>>;
    /// Get dogma attributes.
    fn get_attrs(&self) -> Result<Container<Attr>>;
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> Result<Container<ItemAttr>>;
    /// Get dogma effects.
    fn get_effects(&self) -> Result<Container<Effect>>;
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> Result<Container<ItemEffect>>;
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> Result<Container<FighterAbil>>;
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> Result<Container<ItemFighterAbil>>;
    /// Get dogma buffs.
    fn get_buffs(&self) -> Result<Container<Buff>>;
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> Result<Container<ItemSkillReq>>;
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> Result<Container<MutaItemConv>>;
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> Result<Container<MutaAttrMod>>;
    /// Get version of the data.
    fn get_version(&self) -> Result<String>;
}
