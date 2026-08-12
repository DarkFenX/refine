#[cfg(any(feature = "phb-fs", feature = "phb-http"))]
pub(crate) use string_funcs::cap_len;
pub(crate) use string_funcs::cap_warning_len;

pub(crate) mod data;

mod string_funcs;
