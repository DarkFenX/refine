pub use containers::StatOptionExt;
pub(in crate::stats) use containers::{StatDefOption, StatDefOptionExt};
pub(in crate::stats) use ext_kind::{StatOptionKind, StatOptionRaw, StatOptionResolved};

mod containers;
mod ext_kind;
