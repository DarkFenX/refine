use std::error;
use std::fmt;
use std::result;

use super::data::{
    Container, DgmAttr, DgmBuff, DgmEffect, DgmMutaAttr, DgmMutaType, DgmTypeAttr, DgmTypeEffect, FtrAbil, FtrTypeAbil,
    InvGroup, InvType, SkillReq,
};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait DataHandler: fmt::Debug {
    fn get_invtypes(&self) -> Result<Container<InvType>>;
    fn get_invgroups(&self) -> Result<Container<InvGroup>>;
    fn get_dgmattrs(&self) -> Result<Container<DgmAttr>>;
    fn get_dgmtypeattrs(&self) -> Result<Container<DgmTypeAttr>>;
    fn get_dgmeffects(&self) -> Result<Container<DgmEffect>>;
    fn get_dgmtypeeffects(&self) -> Result<Container<DgmTypeEffect>>;
    fn get_dgmmutatypes(&self) -> Result<Container<DgmMutaType>>;
    fn get_dgmmutaattrs(&self) -> Result<Container<DgmMutaAttr>>;
    fn get_dgmbuffs(&self) -> Result<Container<DgmBuff>>;
    fn get_ftrabils(&self) -> Result<Container<FtrAbil>>;
    fn get_ftrtypeabils(&self) -> Result<Container<FtrTypeAbil>>;
    fn get_skillreqs(&self) -> Result<Container<SkillReq>>;
    fn get_version(&self) -> Result<String>;
}
