use crate::{
    defs::{SolFitId, OF},
    ec,
    err::basic::FitFoundError,
    sol::{
        svc::{
            calc::SolCalc,
            rest::{SolRest, SolStatResource},
        },
        uad::SolUad,
    },
    util::round,
};

impl SolRest {
    pub(in crate::sol::svc) fn get_fit_stats_cpu(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit_id: &SolFitId,
    ) -> Result<SolStatResource, FitFoundError> {
        let ship_id = uad.fits.get_fit(fit_id)?.ship;
        let output = match ship_id {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, &ec::attrs::CPU_OUTPUT) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => OF(0.0),
            },
            None => OF(0.0),
        };
        let used = self
            .data
            .get(fit_id)
            .unwrap()
            .mods_online
            .iter()
            .filter_map(|i| calc.get_item_attr_val(uad, i, &ec::attrs::CPU).ok().map(|v| v.extra))
            .sum();
        // Round possible float errors despite individual use values being rounded
        let used = round(used, 2);
        let stats = SolStatResource::new(used, output);
        Ok(stats)
    }
}
