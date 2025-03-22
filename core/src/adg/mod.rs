//! Adapted data generator

use crate::{
    ad,
    adg::{data::GData, eff_abil::get_abil_effect, support::GSupport},
    ed,
    src::SrcInitError,
};

mod data;
mod eff_abil;
mod flow;
mod rels;
mod support;

/// Fetch EVE data and generate adapted data out of it
#[tracing::instrument(name = "adg", level = "trace", skip_all)]
pub(crate) fn generate_adapted_data(e_handler: &dyn ed::EveDataHandler) -> Result<ad::AData, SrcInitError> {
    let mut g_data = GData::new();
    let mut g_supp = GSupport::new();
    let mut a_data = ad::AData::new();
    flow::fetch_data(e_handler, &mut g_data).map_err(|e| SrcInitError::EveDataFetchFailed(e.to_string()))?;
    flow::dedup_pks(&mut g_data);
    flow::normalize(&mut g_data);
    g_supp.fill(&g_data);
    flow::clean_unused(&mut g_data, &g_supp).map_err(|e| SrcInitError::EveDataCleanupFailed(e.to_string()))?;
    flow::validate(&mut g_data, &g_supp);
    flow::convert(&g_data, &g_supp, &mut a_data);
    flow::customize(&mut a_data);
    flow::fill_extra_data(&mut a_data);
    Ok(a_data)
}
