use crate::{
    defs::{EAttrId, OF},
    ec,
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatResource, SolVastFitData},
        },
        uad::{fit::SolFit, SolUad},
    },
    util::round,
};

impl SolVastFitData {
    pub(in crate::sol::svc::vast) fn get_stats_cpu(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatResource {
        self.get_stats_online_mods(uad, calc, fit, &ec::attrs::CPU, &ec::attrs::CPU_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn get_stats_pg(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatResource {
        self.get_stats_online_mods(uad, calc, fit, &ec::attrs::POWER, &ec::attrs::POWER_OUTPUT)
    }
    fn get_stats_online_mods(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        item_attr_id: &EAttrId,
        ship_attr_id: &EAttrId,
    ) -> SolStatResource {
        let output = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, ship_attr_id) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => OF(0.0),
            },
            None => OF(0.0),
        };
        let used = self
            .mods_online
            .iter()
            .filter_map(|i| calc.get_item_attr_val(uad, i, item_attr_id).ok().map(|v| v.extra))
            .sum();
        // Round possible float errors despite individual use values being rounded
        let used = round(used, 2);
        SolStatResource::new(used, output)
    }
}
