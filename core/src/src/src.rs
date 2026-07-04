use std::sync::Arc;

use super::{error::SrcInitError, info::SrcInfo, prepare::prepare_adapted_data};
use crate::{ad::AdaptedDataCacher, ed::EveDataHandler, rd::RData};

/// Data source.
///
/// Data source is a top-level entity which manages EVE data handler and adapted data cacher to do
/// necessary preparations and expose processed data to solar system and its services.
// Under the hood it's an entity which builds runtime data container, and then provides access to
// its contents
#[derive(Clone)]
pub struct Src {
    pub(crate) r_data: Arc<RData>,
    info: SrcInfo,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: &dyn EveDataHandler,
        ad_cacher: Option<&mut Box<dyn AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let (a_data, info) = prepare_adapted_data(ed_handler, ad_cacher)?;
        let r_data = RData::from_a_data(a_data);
        Ok(Self {
            r_data: Arc::new(r_data),
            info,
        })
    }
}
