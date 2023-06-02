use std::fmt;

use crate::edt::{
    EAttr, EBuff, EEffect, EFighterAbil, EItem, EItemAttr, EItemEffect, EItemFighterAbil, EItemGroup, EItemSkillReq,
    EMutaAttrMod, EMutaItemConv,
};

use super::{cont::Container, Result};

/// EVE data handler interface definition.
///
/// Please be aware that this interface is not expected to be stable. Whenever CCP significantly
/// change the EVE data format, the interface has to change as well.
///
/// All the methods required by this trait should return an error only when it is impossible to
/// fetch the data altogether. In case of a less impactful error (such as inability to deserialize
/// one specific item within a big array of data), the error should be recorded as a meaningful
/// warning message and stored in [`Container::warns`](self::Container::warns).
pub trait EveDataHandler: fmt::Debug {
    /// Get item types.
    fn get_items(&self) -> Result<Container<EItem>>;
    /// Get item groups.
    fn get_item_groups(&self) -> Result<Container<EItemGroup>>;
    /// Get dogma attributes.
    fn get_attrs(&self) -> Result<Container<EAttr>>;
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> Result<Container<EItemAttr>>;
    /// Get dogma effects.
    fn get_effects(&self) -> Result<Container<EEffect>>;
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> Result<Container<EItemEffect>>;
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> Result<Container<EFighterAbil>>;
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> Result<Container<EItemFighterAbil>>;
    /// Get dogma buffs.
    fn get_buffs(&self) -> Result<Container<EBuff>>;
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> Result<Container<EItemSkillReq>>;
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> Result<Container<EMutaItemConv>>;
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> Result<Container<EMutaAttrMod>>;
    /// Get version of the data.
    fn get_version(&self) -> Result<String>;
}
