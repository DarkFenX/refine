use crate::{
    ad::{ADataGenerator, ADataGeneratorError},
    ed::{EDataCont, EveDataHandler},
    util::LibNamed,
};

const MAX_WARNS: usize = 5;

impl ADataGenerator {
    pub(in crate::ad::generator) fn fetch_data(
        &mut self,
        ed_handler: &dyn EveDataHandler,
    ) -> Result<(), ADataGeneratorError> {
        tracing::debug!("fetching EVE data");
        self.e_data = ed_handler
            .get_data()
            .map_err(|e| ADataGeneratorError::DataFetchFailed(e.to_string()))?;
        report_warnings(&self.e_data.items);
        report_warnings(&self.e_data.groups);
        report_warnings(&self.e_data.item_lists);
        report_warnings(&self.e_data.attrs);
        report_warnings(&self.e_data.item_attrs);
        report_warnings(&self.e_data.effects);
        report_warnings(&self.e_data.item_effects);
        report_warnings(&self.e_data.abils);
        report_warnings(&self.e_data.item_abils);
        report_warnings(&self.e_data.buffs);
        report_warnings(&self.e_data.space_comps);
        report_warnings(&self.e_data.item_srqs);
        report_warnings(&self.e_data.muta_items);
        report_warnings(&self.e_data.muta_attrs);
        Ok(())
    }
}

fn report_warnings<T>(data_cont: &EDataCont<T>)
where
    T: LibNamed,
{
    let warn_count = data_cont.warns.len();
    if warn_count > 0 {
        tracing::warn!(
            "{} warnings encountered during fetching of {}, showing up to {}:",
            warn_count,
            T::lib_get_name(),
            MAX_WARNS
        );
        for warn_msg in data_cont.warns.iter().take(MAX_WARNS) {
            tracing::warn!("{warn_msg}");
        }
    }
}
