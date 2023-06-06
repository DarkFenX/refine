use log;

use crate::{
    adg::GData,
    ed::{self, EveDataHandler},
    util::{IntError, IntResult, Named},
};

const MAX_WARNS: usize = 5;

/// Fetch data from a data handler into a data vec, and report warnings, if any were encountered.
fn fetch_data_vec<S, F, T>(e_handler: &S, func: F, vec: &mut Vec<T>) -> IntResult<()>
where
    S: ?Sized + EveDataHandler,
    F: Fn(&S) -> ed::EResult<ed::EDataCont<T>>,
    T: Named,
{
    log::debug!("fetching {}", T::get_name());
    let e_cont = func(e_handler).map_err(|e| IntError::new(e.to_string()))?;
    vec.extend(e_cont.data);
    let warn_amt = e_cont.warns.len();
    if warn_amt > 0 {
        log::warn!(
            "{} warnings encountered during fetching of {}, showing up to {}:",
            warn_amt,
            T::get_name(),
            MAX_WARNS
        );
        for warn_msg in e_cont.warns.iter().take(MAX_WARNS) {
            log::warn!("{warn_msg}");
        }
    }
    Ok(())
}

pub(in crate::adg) fn fetch_data(e_handler: &dyn EveDataHandler, g_data: &mut GData) -> IntResult<()> {
    fetch_data_vec(e_handler, EveDataHandler::get_items, &mut g_data.items)?;
    fetch_data_vec(e_handler, EveDataHandler::get_item_groups, &mut g_data.groups)?;
    fetch_data_vec(e_handler, EveDataHandler::get_attrs, &mut g_data.attrs)?;
    fetch_data_vec(e_handler, EveDataHandler::get_item_attrs, &mut g_data.item_attrs)?;
    fetch_data_vec(e_handler, EveDataHandler::get_effects, &mut g_data.effects)?;
    fetch_data_vec(e_handler, EveDataHandler::get_item_effects, &mut g_data.item_effects)?;
    fetch_data_vec(e_handler, EveDataHandler::get_fighter_abils, &mut g_data.abils)?;
    fetch_data_vec(
        e_handler,
        EveDataHandler::get_item_fighter_abils,
        &mut g_data.item_abils,
    )?;
    fetch_data_vec(e_handler, EveDataHandler::get_buffs, &mut g_data.buffs)?;
    fetch_data_vec(e_handler, EveDataHandler::get_item_skill_reqs, &mut g_data.item_srqs)?;
    fetch_data_vec(e_handler, EveDataHandler::get_muta_item_convs, &mut g_data.muta_items)?;
    fetch_data_vec(e_handler, EveDataHandler::get_muta_attr_mods, &mut g_data.muta_attrs)?;
    Ok(())
}
