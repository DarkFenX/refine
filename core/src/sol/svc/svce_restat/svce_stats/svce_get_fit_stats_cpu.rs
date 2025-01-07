use itertools::Itertools;

use crate::{
    defs::{SolFitId, OF},
    ec,
    err::basic::FitFoundError,
    sol::{
        svc::{svce_restat::SolStatResource, SolSvc},
        uad::SolUad,
    },
    util::round,
};

impl SolSvc {
    pub(in crate::sol::svc) fn get_fit_stats_cpu(
        &mut self,
        uad: &SolUad,
        fit_id: &SolFitId,
    ) -> Result<SolStatResource, FitFoundError> {
        let ship_id = uad.fits.get_fit(fit_id)?.ship;
        let output = match ship_id {
            Some(ship_id) => match self.calc_get_item_attr_val(uad, &ship_id, &ec::attrs::CPU_OUTPUT) {
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
                self.calc_get_item_attr_val(uad, i, &ec::attrs::CPU)
                    .ok()
                    .map(|v| v.extra)
            })
            .sum();
        // Round possible float errors despite individual use values being rounded
        let used = round(used, 2);
        let stats = SolStatResource::new(used, output);
        Ok(stats)
    }
}
