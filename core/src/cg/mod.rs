//! Cache generator.

use data::{CGData, Support};

use crate::{ch, dh::DataHandler, Result};

mod clean;
mod conv;
mod data;
mod fetch;
mod pk;
mod valid;

pub(crate) fn generate_cache(data_handler: &dyn DataHandler) -> Result<ch::CHData> {
    let mut cg_data = CGData::new();
    let mut warns = Vec::new();
    let mut supp = Support::new();
    let mut ch_data = ch::CHData::new();
    fetch::fetch_data(data_handler, &mut cg_data)?;
    pk::dedup_pks(&mut cg_data, &mut warns);
    supp.post_pk(&cg_data);
    clean::clean_unused(&mut cg_data, &supp)?;
    valid::validate(&mut cg_data, &supp, &mut warns);
    conv::convert(&cg_data, &supp, &mut warns, &mut ch_data);
    Ok(ch_data)
}
