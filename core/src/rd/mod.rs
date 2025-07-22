//! RD stands for Runtime Data.
//!
//! This module contains all the entities enriched during runtime.

use container::REntityContainer;
pub(crate) use data::{RAttr, RAttrKey, RBuff, RBuffKey, RBuffModifier, RData, REffectKey, RItemKey, RMuta, RMutaKey};

mod container;
mod data;
