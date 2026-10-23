use crate::{
    ad::{ADataGenerator, ADataWarnings},
    ed::{EData, EDataCont, EveDataHandler, err::EveDataHandlerError},
    util::LibNamed,
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn fetch_data(
        &mut self,
        ed_handler: &EveDataHandler,
    ) -> Result<(), EveDataHandlerError> {
        self.e_data = ed_handler.get_impl().get_data()?;
        let EData {
            items,
            groups,
            item_lists,
            attrs,
            item_attrs,
            effects,
            item_effects,
            abils,
            item_abils,
            buffs,
            item_buffs,
            muta_items,
            muta_attrs,
        } = &mut self.e_data;
        record_warnings(items, &mut self.a_data.warnings);
        record_warnings(groups, &mut self.a_data.warnings);
        record_warnings(item_lists, &mut self.a_data.warnings);
        record_warnings(attrs, &mut self.a_data.warnings);
        record_warnings(item_attrs, &mut self.a_data.warnings);
        record_warnings(effects, &mut self.a_data.warnings);
        record_warnings(item_effects, &mut self.a_data.warnings);
        record_warnings(abils, &mut self.a_data.warnings);
        record_warnings(item_abils, &mut self.a_data.warnings);
        record_warnings(buffs, &mut self.a_data.warnings);
        record_warnings(item_buffs, &mut self.a_data.warnings);
        record_warnings(muta_items, &mut self.a_data.warnings);
        record_warnings(muta_attrs, &mut self.a_data.warnings);
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
            .take(warning_limit)
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
