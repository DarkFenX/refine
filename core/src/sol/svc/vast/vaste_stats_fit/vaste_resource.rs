use crate::{
    defs::{AttrVal, EAttrId, OF},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{fit::SolFit, SolUad},
    },
    util::round,
};

pub struct SolStatRes {
    pub used: AttrVal,
    pub output: AttrVal,
}
impl SolStatRes {
    pub(in crate::sol::svc::vast) fn new(used: AttrVal, output: AttrVal) -> Self {
        SolStatRes { used, output }
    }
}

impl SolVastFitData {
    pub(in crate::sol::svc::vast) fn get_stats_cpu(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        self.get_stats_online_mods(uad, calc, fit, &ec::attrs::CPU, &ec::attrs::CPU_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn get_stats_pg(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> SolStatRes {
        self.get_stats_online_mods(uad, calc, fit, &ec::attrs::POWER, &ec::attrs::POWER_OUTPUT)
    }
    fn get_stats_online_mods(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        use_attr_id: &EAttrId,
        output_attr_id: &EAttrId,
    ) -> SolStatRes {
        let output = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, output_attr_id) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => OF(0.0),
            },
            None => OF(0.0),
        };
        let used = self
            .mods_online
            .iter()
            .filter_map(|i| calc.get_item_attr_val(uad, i, use_attr_id).ok().map(|v| v.extra))
            .sum();
        // Round possible float errors despite individual use values being rounded
        let used = round(used, 2);
        SolStatRes::new(used, output)
    }
}
