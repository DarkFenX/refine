//! Cache generator.

use data::Data;

use crate::dh::DataHandler;
use crate::util::Result;

mod data;
mod fetch;
mod pk;

pub fn generate_cache(data_handler: &dyn DataHandler) -> Result<()> {
    let mut data = Data::new();
    let mut errs = Vec::new();
    fetch::fetch_data(data_handler, &mut data);
    pk::dedup_pks(&mut data, &mut errs);
    Ok(())
}
