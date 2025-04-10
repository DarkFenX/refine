use crate::{cmd::shared::HValOptions, util::HExecError};

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
        let core_options = self.validation_options.into_core_val_options(core_sol);
        core_sol
            .try_fit_items(fit_id, &self.type_ids, &core_options)
            .map_err(|core_error| match core_error {
                rc::err::TryFitItemsError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
            })
    }
}
