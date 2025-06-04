use crate::{
    cmd::shared::{HValOptions, get_primary_fit},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HTryFitItemsCmd {
    type_ids: Vec<rc::ItemTypeId>,
    validation_options: HValOptions,
}
impl HTryFitItemsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<Vec<rc::ItemTypeId>, HExecError> {
        let core_options = self.validation_options.to_core_val_options();
        let mut primary_fit = get_primary_fit(core_sol, fit_id)?;
        Ok(primary_fit.try_fit_items(&self.type_ids, &core_options))
    }
}
