use itertools::Itertools;
use serde::Deserialize;

use crate::{
    cmd::shared::{HValOptions, get_primary_fit},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HTryFitItemsCmd {
    type_ids: Vec<i32>,
    #[serde(default)]
    validation_options: HValOptions,
}
impl HTryFitItemsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<Vec<rc::ItemTypeId>, HExecError> {
        let mut primary_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_ids = self.type_ids.iter().map(|v| rc::ItemTypeId::from_i32(*v)).collect_vec();
        let core_options = self.validation_options.to_core();
        Ok(primary_fit.try_fit_items(&core_type_ids, &core_options))
    }
}
