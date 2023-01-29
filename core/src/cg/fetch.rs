//! Contains facilities which fetch data from a data handler and store it in a generator-specific
//! container.

use log;

use crate::{
    dh::{self, DataHandler},
    util::Named,
    IntError, IntResult,
};

use super::data::CGData;

const MAX_WARNS: usize = 5;

/// Fetch data from a data handler into a data vec, and report warnings, if any were encountered.
fn fetch_data_vec<S, F, T>(handler: &S, func: F, vec: &mut Vec<T>) -> IntResult<()>
where
    S: ?Sized + DataHandler,
    F: Fn(&S) -> dh::Result<dh::Container<T>>,
    T: Named,
{
    log::debug!("fetching {}", T::get_name());
    let cont = func(handler).map_err(|e| IntError::new(format!("{}", e)))?;
    vec.extend(cont.data);
    let warn_amt = cont.warns.len();
    if warn_amt > 0 {
        log::warn!(
            "{} warnings encountered during fetching of {}, showing up to {}:",
            warn_amt,
            T::get_name(),
            MAX_WARNS
        );
        for warn_msg in cont.warns.iter().take(MAX_WARNS) {
            log::warn!("{}", warn_msg);
        }
    }
    Ok(())
}

pub(super) fn fetch_data(data_handler: &dyn DataHandler, cg_data: &mut CGData) -> IntResult<()> {
    fetch_data_vec(data_handler, DataHandler::get_items, &mut cg_data.items)?;
    fetch_data_vec(data_handler, DataHandler::get_item_groups, &mut cg_data.groups)?;
    fetch_data_vec(data_handler, DataHandler::get_attrs, &mut cg_data.attrs)?;
    fetch_data_vec(data_handler, DataHandler::get_item_attrs, &mut cg_data.item_attrs)?;
    fetch_data_vec(data_handler, DataHandler::get_effects, &mut cg_data.effects)?;
    fetch_data_vec(data_handler, DataHandler::get_item_effects, &mut cg_data.item_effects)?;
    fetch_data_vec(data_handler, DataHandler::get_fighter_abils, &mut cg_data.abils)?;
    fetch_data_vec(
        data_handler,
        DataHandler::get_item_fighter_abils,
        &mut cg_data.item_abils,
    )?;
    fetch_data_vec(data_handler, DataHandler::get_buffs, &mut cg_data.buffs)?;
    fetch_data_vec(data_handler, DataHandler::get_item_skill_reqs, &mut cg_data.item_srqs)?;
    fetch_data_vec(data_handler, DataHandler::get_muta_item_convs, &mut cg_data.muta_items)?;
    fetch_data_vec(data_handler, DataHandler::get_muta_attr_mods, &mut cg_data.muta_attrs)?;
    Ok(())
}
