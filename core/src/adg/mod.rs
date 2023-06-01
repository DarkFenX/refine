//! EVE reefast types generator.

use data::{Data, Support};

use crate::{adh, edh, util::IntResult};

mod clean;
mod conv;
mod data;
mod fetch;
mod pk;
mod valid;

pub(crate) fn generate_erts(data_handler: &dyn edh::EveDataHandler) -> IntResult<adh::Data> {
    let mut erg_data = Data::new();
    let mut supp = Support::new();
    let mut erh_data = adh::Data::new();
    fetch::fetch_data(data_handler, &mut erg_data)?;
    pk::dedup_pks(&mut erg_data);
    supp.post_pk(&erg_data);
    clean::clean_unused(&mut erg_data, &supp)?;
    valid::validate(&mut erg_data, &supp);
    conv::convert(&erg_data, &supp, &mut erh_data);
    Ok(erh_data)
}
