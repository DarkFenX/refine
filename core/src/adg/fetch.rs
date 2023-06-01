//! Contains facilities which fetch data from a data handler and store it in a generator-specific
//! container.

use log;

use crate::{
    edh::{self, EveDataHandler},
    util::{IntError, IntResult, Named},
};

use super::data::Data;

const MAX_WARNS: usize = 5;

/// Fetch data from a data handler into a data vec, and report warnings, if any were encountered.
fn fetch_data_vec<S, F, T>(handler: &S, func: F, vec: &mut Vec<T>) -> IntResult<()>
where
    S: ?Sized + EveDataHandler,
    F: Fn(&S) -> edh::Result<edh::Container<T>>,
    T: Named,
{
    log::debug!("fetching {}", T::get_name());
    let cont = func(handler).map_err(|e| IntError::new(e.to_string()))?;
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

pub(super) fn fetch_data(data_handler: &dyn EveDataHandler, erg_data: &mut Data) -> IntResult<()> {
    fetch_data_vec(data_handler, EveDataHandler::get_items, &mut erg_data.items)?;
    fetch_data_vec(data_handler, EveDataHandler::get_item_groups, &mut erg_data.groups)?;
    fetch_data_vec(data_handler, EveDataHandler::get_attrs, &mut erg_data.attrs)?;
    fetch_data_vec(data_handler, EveDataHandler::get_item_attrs, &mut erg_data.item_attrs)?;
    fetch_data_vec(data_handler, EveDataHandler::get_effects, &mut erg_data.effects)?;
    fetch_data_vec(
        data_handler,
        EveDataHandler::get_item_effects,
        &mut erg_data.item_effects,
    )?;
    fetch_data_vec(data_handler, EveDataHandler::get_fighter_abils, &mut erg_data.abils)?;
    fetch_data_vec(
        data_handler,
        EveDataHandler::get_item_fighter_abils,
        &mut erg_data.item_abils,
    )?;
    fetch_data_vec(data_handler, EveDataHandler::get_buffs, &mut erg_data.buffs)?;
    fetch_data_vec(
        data_handler,
        EveDataHandler::get_item_skill_reqs,
        &mut erg_data.item_srqs,
    )?;
    fetch_data_vec(
        data_handler,
        EveDataHandler::get_muta_item_convs,
        &mut erg_data.muta_items,
    )?;
    fetch_data_vec(
        data_handler,
        EveDataHandler::get_muta_attr_mods,
        &mut erg_data.muta_attrs,
    )?;
    Ok(())
}
