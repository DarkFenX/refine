//! Cache generator.

use data::{Data, Support};

use crate::{ch, dh, IntResult};

mod clean;
mod conv;
mod data;
mod fetch;
mod pk;
mod valid;

pub(crate) fn generate_cache(data_handler: &dyn dh::DataHandler) -> IntResult<ch::Data> {
    let mut cg_data = Data::new();
    let mut supp = Support::new();
    let mut ch_data = ch::Data::new();
    fetch::fetch_data(data_handler, &mut cg_data)?;
    pk::dedup_pks(&mut cg_data);
    supp.post_pk(&cg_data);
    clean::clean_unused(&mut cg_data, &supp)?;
    valid::validate(&mut cg_data, &supp);
    conv::convert(&cg_data, &supp, &mut ch_data);
    Ok(ch_data)
}
