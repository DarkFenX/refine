use crate::{
    defs::{Amount, EAttrId, SolItemId},
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
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::UPGRADE_SLOTS_LEFT,
            fit.rigs.len() as Amount,
        )
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
            fit.ship,
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
            fit.character,
            &ec::attrs::MAX_ACTIVE_DRONES,
            self.drones_online_bandwidth.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_TUBES,
            self.fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_support_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_SUPPORT_SLOTS,
            self.support_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_light_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_LIGHT_SLOTS,
            self.light_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_heavy_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_HEAVY_SLOTS,
            self.heavy_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_support_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            self.standup_support_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_light_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_STANDUP_LIGHT_SLOTS,
            self.standup_light_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_heavy_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::FTR_STANDUP_HEAVY_SLOTS,
            self.standup_heavy_fighters_online.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_turret_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::TURRET_SLOTS_LEFT,
            self.mods_turret.len() as Amount,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_launcher_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        self.get_stats_slots(
            uad,
            calc,
            fit.ship,
            &ec::attrs::LAUNCHER_SLOTS_LEFT,
            self.mods_launcher.len() as Amount,
        )
    }
    // Private methods
    fn get_stats_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        output_item_id: Option<SolItemId>,
        output_attr_id: &EAttrId,
        user_amount: Amount,
    ) -> SolStatSlot {
        let total = match output_item_id {
            Some(output_item_id) => match calc.get_item_attr_val(uad, &output_item_id, output_attr_id) {
                Ok(attr_val) => Some(attr_val.extra.into_inner().round() as Amount),
                Err(_) => None,
            },
            None => None,
        };
        SolStatSlot::new(user_amount, total)
    }
}
