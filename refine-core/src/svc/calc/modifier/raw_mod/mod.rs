pub(in crate::svc::calc) use affector::{RawModAffector, RawModItemDependent};
pub(in crate::svc::calc) use id::{RawModId, RawModIdKind, RawModSlot};
pub(in crate::svc::calc) use raw_mod::RawModifier;
pub(crate) use template_storage::RawModTemplates;

mod affector;
mod id;
mod raw_mod;
mod template_storage;
