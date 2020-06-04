//! Cache generator.

use data::{Data, Support};

use crate::{dh::DataHandler, util::Result};

mod clean;
mod conv;
mod data;
mod fetch;
mod pk;
mod valid;

pub fn generate_cache(data_handler: &dyn DataHandler) -> Result<()> {
    let mut data = Data::new();
    let mut warns = Vec::new();
    let mut supp = Support::new();
    fetch::fetch_data(data_handler, &mut data)?;
    pk::dedup_pks(&mut data, &mut warns);
    supp.post_pk(&data);
    clean::clean_unused(&mut data, &supp)?;
    valid::validate(&mut data, &supp, &mut warns);
    conv::convert(&data);
    Ok(())
}
