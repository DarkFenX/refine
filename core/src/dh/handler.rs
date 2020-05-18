use std::error;
use std::result;

use super::data::{Container, EveType};

pub type Result<T> = result::Result<Container<T>, Box<dyn error::Error>>;

pub trait Handler {
    fn get_evetypes(&self) -> Result<EveType>;
}
