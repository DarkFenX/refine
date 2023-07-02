//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. Adapted data types are assumed to be read-only by
//! the components, anything mutable is built on top of them.

pub use data::{
    AAttr, AAttrMod, ABuff, ABuffAttrMod, AData, AEffect, AItem, AItemEffData, AMuta, AMutaAttrRange, ArcAttr, ArcBuff,
    ArcEffect, ArcItem, ArcMuta, ModAfeeFilter, ModBuildStatus, ModSrq,
};
pub use handler::AdaptedDataHandler;
pub use result::AResult;

mod data;
mod handler;
mod result;
