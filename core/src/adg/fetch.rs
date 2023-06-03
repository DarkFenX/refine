use log;

use crate::{
    adg::GData,
    ed::{self, EveDataHandler},
    util::{IntError, IntResult, Named},
};

const MAX_WARNS: usize = 5;

/// Fetch data from a data handler into a data vec, and report warnings, if any were encountered.
fn fetch_data_vec<S, F, T>(ehandler: &S, func: F, vec: &mut Vec<T>) -> IntResult<()>
where
    S: ?Sized + EveDataHandler,
    F: Fn(&S) -> ed::EResult<ed::EDataCont<T>>,
    T: Named,
{
    log::debug!("fetching {}", T::get_name());
    let cont = func(ehandler).map_err(|e| IntError::new(e.to_string()))?;
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

pub(in crate::adg) fn fetch_data(ehandler: &dyn EveDataHandler, gdata: &mut GData) -> IntResult<()> {
    fetch_data_vec(ehandler, EveDataHandler::get_items, &mut gdata.items)?;
    fetch_data_vec(ehandler, EveDataHandler::get_item_groups, &mut gdata.groups)?;
    fetch_data_vec(ehandler, EveDataHandler::get_attrs, &mut gdata.attrs)?;
    fetch_data_vec(ehandler, EveDataHandler::get_item_attrs, &mut gdata.item_attrs)?;
    fetch_data_vec(ehandler, EveDataHandler::get_effects, &mut gdata.effects)?;
    fetch_data_vec(ehandler, EveDataHandler::get_item_effects, &mut gdata.item_effects)?;
    fetch_data_vec(ehandler, EveDataHandler::get_fighter_abils, &mut gdata.abils)?;
    fetch_data_vec(ehandler, EveDataHandler::get_item_fighter_abils, &mut gdata.item_abils)?;
    fetch_data_vec(ehandler, EveDataHandler::get_buffs, &mut gdata.buffs)?;
    fetch_data_vec(ehandler, EveDataHandler::get_item_skill_reqs, &mut gdata.item_srqs)?;
    fetch_data_vec(ehandler, EveDataHandler::get_muta_item_convs, &mut gdata.muta_items)?;
    fetch_data_vec(ehandler, EveDataHandler::get_muta_attr_mods, &mut gdata.muta_attrs)?;
    Ok(())
}
