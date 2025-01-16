use crate::{
    defs::{Amount, EAttrId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{fit::SolFit, SolUad},
    },
};

pub struct SolStatSlot {
    pub used: Amount,
    pub total: Option<Amount>,
}
impl SolStatSlot {
    pub(in crate::sol::svc::vast) fn new(used: Amount, total: Option<Amount>) -> Self {
        SolStatSlot { used, total }
    }
}

impl SolVastFitData {
    // Public methods
    pub(in crate::sol::svc::vast) fn get_stats_rig_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(uad, calc, fit, &ec::attrs::UPGRADE_SLOTS_LEFT, fit.rigs.len() as Amount)
    }
    pub(in crate::sol::svc::vast) fn get_stats_subsystem_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit,
            &ec::attrs::MAX_SUBSYSTEMS,
            fit.subsystems.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_drones(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit,
            &ec::attrs::MAX_ACTIVE_DRONES,
            self.drones_online_bandwidth.len() as Amount,
        )
    }
    // Private methods
    fn get_stats_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        output_attr_id: &EAttrId,
        user_amount: Amount,
    ) -> SolStatSlot {
        let total = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, output_attr_id) {
                Ok(attr_val) => Some(attr_val.extra.into_inner().round() as Amount),
                Err(_) => None,
            },
            None => None,
        };
        SolStatSlot::new(user_amount, total)
    }
}
