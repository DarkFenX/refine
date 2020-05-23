use std::error;
use std::fmt;
use std::result;

use super::data::{Container, DgmAttr, DgmBuff, DgmEffect, InvGroup, InvType, FtrAbil, FtrTypeAbil};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait Handler: fmt::Debug {
    fn get_invtypes(&self) -> Result<Container<InvType>>;
    fn get_invgroups(&self) -> Result<Container<InvGroup>>;
    fn get_dgmattrs(&self) -> Result<Container<DgmAttr>>;
    fn get_dgmeffects(&self) -> Result<Container<DgmEffect>>;
    fn get_dgmbuffs(&self) -> Result<Container<DgmBuff>>;
    fn get_ftrabils(&self) -> Result<Container<FtrAbil>>;
    fn get_ftrtypeabils(&self) -> Result<Container<FtrTypeAbil>>;
    fn get_version(&self) -> Result<String>;
}
