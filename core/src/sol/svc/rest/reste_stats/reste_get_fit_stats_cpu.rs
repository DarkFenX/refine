use crate::{
    defs::{EAttrId, SolFitId, OF},
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
        self.get_fit_stats_online_mods(uad, calc, fit_id, &ec::attrs::CPU, &ec::attrs::CPU_OUTPUT)
    }
    pub(in crate::sol::svc) fn get_fit_stats_pg(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit_id: &SolFitId,
    ) -> Result<SolStatResource, FitFoundError> {
        self.get_fit_stats_online_mods(uad, calc, fit_id, &ec::attrs::POWER, &ec::attrs::POWER_OUTPUT)
    }
    fn get_fit_stats_online_mods(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit_id: &SolFitId,
        item_attr_id: &EAttrId,
        ship_attr_id: &EAttrId,
    ) -> Result<SolStatResource, FitFoundError> {
        let ship_id = uad.fits.get_fit(fit_id)?.ship;
        let output = match ship_id {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, ship_attr_id) {
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
            .filter_map(|i| calc.get_item_attr_val(uad, i, item_attr_id).ok().map(|v| v.extra))
            .sum();
        // Round possible float errors despite individual use values being rounded
        let used = round(used, 2);
        let stats = SolStatResource::new(used, output);
        Ok(stats)
    }
}
