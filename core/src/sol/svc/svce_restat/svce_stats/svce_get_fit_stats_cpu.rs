use crate::{
    defs::{SolFitId, OF},
    ec,
    err::basic::FitFoundError,
    sol::{
        svc::{svce_restat::SolStatResource, SolSvcs},
        SolView,
    },
};
use itertools::Itertools;

impl SolSvcs {
    pub(in crate::sol::svc) fn get_fit_stats_cpu(
        &mut self,
        sol_view: &SolView,
        fit_id: &SolFitId,
    ) -> Result<SolStatResource, FitFoundError> {
        let ship_id = sol_view.fits.get_fit(fit_id)?.ship;
        let output = match ship_id {
            Some(ship_id) => match self.calc_get_item_attr_val(sol_view, &ship_id, &ec::attrs::CPU_OUTPUT) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => OF(0.0),
            },
            None => OF(0.0),
        };
        let users = self
            .restat
            .data
            .get(fit_id)
            .unwrap()
            .mods_online
            .iter()
            .map(|v| *v)
            .collect_vec();
        let used = users
            .iter()
            .filter_map(|i| {
                self.calc_get_item_attr_val(sol_view, i, &ec::attrs::CPU)
                    .ok()
                    .map(|v| v.extra)
            })
            .sum();
        let stats = SolStatResource::new(used, output);
        Ok(stats)
    }
}
