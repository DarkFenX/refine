use crate::{
    ad::{ADataGenerator, ADataWarnings, err::ADataGeneratorError},
    ed::{EDataCont, EveDataHandler},
    util::LibNamed,
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn fetch_data(
        &mut self,
        ed_handler: &dyn EveDataHandler,
    ) -> Result<(), ADataGeneratorError> {
        self.e_data = ed_handler
            .get_data()
            .map_err(|e| ADataGeneratorError::DataFetchFailed(e.to_string()))?;
        record_warnings(&mut self.e_data.items, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.groups, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.item_lists, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.attrs, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.item_attrs, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.effects, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.item_effects, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.abils, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.item_abils, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.buffs, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.space_comps, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.item_srqs, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.muta_items, &mut self.a_data.warnings);
        record_warnings(&mut self.e_data.muta_attrs, &mut self.a_data.warnings);
        Ok(())
    }
}

fn record_warnings<T>(e_cont: &mut EDataCont<T>, a_warnings: &mut ADataWarnings)
where
    T: LibNamed,
{
    let warning_count = e_cont.warnings.len();
    let warning_limit = 5;
    a_warnings.data_fetch.extend(
        e_cont
            .warnings
            .drain(..)
            .map(|v| format!("failed to fetch {}: {}", T::lib_get_name(), v)),
    );
    if warning_count > warning_limit {
        let warning = format!(
            "failed to fetch {}: <{} more warnings hidden>",
            T::lib_get_name(),
            warning_count - warning_limit
        );
        a_warnings.data_fetch.push(warning);
    }
}
