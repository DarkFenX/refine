use crate::{
    defs::Amount,
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
    pub(in crate::sol::svc::vast) fn get_stats_rig_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let total = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, &ec::attrs::UPGRADE_SLOTS_LEFT) {
                Ok(attr_val) => Some(attr_val.extra.into_inner() as Amount),
                Err(_) => None,
            },
            None => None,
        };
        let used = fit.rigs.len() as Amount;
        SolStatSlot::new(used, total)
    }
}
