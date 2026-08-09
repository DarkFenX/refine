//! Adapted data generator

use eff_abil::get_abil_effect;
pub use error::ADataGeneratorError;
pub use flow::ADataGeneratorCleanupError;
pub(crate) use generator::ADataGenerator;
use support::AdgSupport;

mod eff_abil;
mod error;
mod flow;
mod generator;
mod rels;
mod support;
