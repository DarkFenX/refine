//! Adapted data generator

use eff_abil::get_abil_effect;
pub(crate) use error::ADataGeneratorError;
pub(crate) use generator::ADataGenerator;
use support::AdgSupport;

mod eff_abil;
mod error;
mod flow;
mod generator;
mod rels;
mod support;
