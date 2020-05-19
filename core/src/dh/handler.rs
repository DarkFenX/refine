use std::error;
use std::fmt;
use std::result;

use super::data::{Container, EveGroup, EveType};

pub type Result<T> = result::Result<Container<T>, Box<dyn error::Error>>;

pub trait Handler: fmt::Debug {
    fn get_evetypes(&self) -> Result<EveType>;
    fn get_evegroups(&self) -> Result<EveGroup>;
}
