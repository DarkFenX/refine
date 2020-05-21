use std::error;
use std::fmt;
use std::result;

use super::data::{Buff, Container, DgmAttr, DgmEffect, EveGroup, EveType, FighterAbil, TypeFighterAbil};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait Handler: fmt::Debug {
    fn get_evetypes(&self) -> Result<Container<EveType>>;
    fn get_evegroups(&self) -> Result<Container<EveGroup>>;
    fn get_dgmattrs(&self) -> Result<Container<DgmAttr>>;
    fn get_dgmeffects(&self) -> Result<Container<DgmEffect>>;
    fn get_buffs(&self) -> Result<Container<Buff>>;
    fn get_fighterabils(&self) -> Result<Container<FighterAbil>>;
    fn get_typefighterabils(&self) -> Result<Container<TypeFighterAbil>>;
    fn get_version(&self) -> Result<String>;
}
