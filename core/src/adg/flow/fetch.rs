use crate::{
    ed,
    util::{Named, StrMsgError},
};

const MAX_WARNS: usize = 5;

/// Report warnings.
fn report_warnings<T>(data_cont: &ed::EDataCont<T>)
where
    T: Named,
{
    let warn_count = data_cont.warns.len();
    if warn_count > 0 {
        tracing::warn!(
            "{} warnings encountered during fetching of {}, showing up to {}:",
            warn_count,
            T::get_name(),
            MAX_WARNS
        );
        for warn_msg in data_cont.warns.iter().take(MAX_WARNS) {
            tracing::warn!("{warn_msg}");
        }
    }
}

pub(in crate::adg) fn fetch_data(e_handler: &dyn ed::EveDataHandler) -> Result<ed::EData, StrMsgError> {
    let e_data = e_handler.get_data().map_err(|e| StrMsgError { msg: e.to_string() })?;
    report_warnings(&e_data.items);
    report_warnings(&e_data.groups);
    report_warnings(&e_data.item_lists);
    report_warnings(&e_data.attrs);
    report_warnings(&e_data.item_attrs);
    report_warnings(&e_data.effects);
    report_warnings(&e_data.item_effects);
    report_warnings(&e_data.abils);
    report_warnings(&e_data.item_abils);
    report_warnings(&e_data.buffs);
    report_warnings(&e_data.space_comps);
    report_warnings(&e_data.item_srqs);
    report_warnings(&e_data.muta_items);
    report_warnings(&e_data.muta_attrs);
    Ok(e_data)
}
