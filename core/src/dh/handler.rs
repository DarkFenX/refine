use std::error;
use std::fmt;
use std::result;

use super::data::{Container, EveGroup, EveType};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait Handler: fmt::Debug {
    fn get_evetypes(&self) -> Result<Container<EveType>>;
    fn get_evegroups(&self) -> Result<Container<EveGroup>>;
    fn get_version(&self) -> Result<String>;
}
