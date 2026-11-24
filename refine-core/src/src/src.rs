use std::sync::Arc;

use crate::{
    rd::RData,
    src::{SrcOrigin, SrcWarnings},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Data source.
///
/// Data source is a top-level entity which manages EVE data handler and adapted data cacher to do
/// necessary preparations and expose processed data to solar system and its services.
/// Under the hood it's an entity which builds runtime data container, and then provides access to
/// its contents
pub struct Src {
    pub(crate) r_data: Arc<RData>,
    pub(in crate::src) origin: SrcOrigin,
    pub(in crate::src) warnings: SrcWarnings,
}
