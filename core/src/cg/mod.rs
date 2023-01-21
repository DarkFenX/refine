//! Cache generator.

use data::{Data, Support};

use crate::{ch, dh::DataHandler, util::Result};

mod clean;
mod conv;
mod data;
mod fetch;
mod pk;
mod valid;

// TODO: remove pub, it's here only for development process
pub fn generate_cache(data_handler: &dyn DataHandler) -> Result<ch::Container> {
    let mut data = Data::new();
    let mut warns = Vec::new();
    let mut supp = Support::new();
    let mut cont = ch::Container::new();
    fetch::fetch_data(data_handler, &mut data)?;
    pk::dedup_pks(&mut data, &mut warns);
    supp.post_pk(&data);
    clean::clean_unused(&mut data, &supp)?;
    valid::validate(&mut data, &supp, &mut warns);
    conv::convert(&data, &supp, &mut warns, &mut cont);
    Ok(cont)
}
