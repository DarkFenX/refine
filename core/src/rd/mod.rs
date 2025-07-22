//! RD stands for Runtime Data.
//!
//! This module contains all the entities enriched during runtime.

use container::REntityContainer;
pub(crate) use data::{RAttr, RAttrKey, RBuffKey, RData, REffectKey, RItemKey, RMutaKey};

mod container;
mod data;
