use crate::{
    Charge, Count, FitId, ItemCommon, ItemSpoolInfo, ModRack, ModuleState, OptionalReload, RackPos, Spool,
    api::{ItemSealed, active_stat_prepare},
    misc::InfCount,
    sol::SolCore,
    stats::{StatItemChargeOptions, StatItemStateOptions},
    ud::{UItemId, UModule},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Module<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Module<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_module(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> ModuleState {
        get_u_module(self.sol, self.uid).get_module_state()
    }
    pub fn get_rack(&self) -> ModRack {
        get_u_module(self.sol, self.uid).get_rack()
    }
    pub fn get_pos(&self) -> RackPos {
        get_u_module(self.sol, self.uid).get_pos()
    }
    pub fn get_charge(&mut self) -> Option<Charge<'_>> {
        get_u_module(self.sol, self.uid)
            .get_charge_uid()
            .map(|charge_uid| Charge::new(self.sol, charge_uid))
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_u_module(self.sol, self.uid).get_charge_count(&self.sol.u_data)
    }
    pub fn get_spool(&self) -> Spool {
        self.sol.u_data.get_item_spool(self.uid, None)
    }
    pub fn get_spool_override(&self) -> Option<Spool> {
        get_u_module(self.sol, self.uid).get_spool_override()
    }
    pub fn get_optional_reload(&self) -> OptionalReload {
        self.sol.u_data.get_item_optional_reload(self.uid, None)
    }
    pub fn get_optional_reload_override(&self) -> Option<OptionalReload> {
        get_u_module(self.sol, self.uid).get_optional_reload_override()
    }
    pub fn get_charged_cycle_count(&mut self) -> Option<Count> {
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, StatItemStateOptions::Switch);
        self.sol.reuse.cseq_cache.clear();
        let result =
            match self
                .sol
                .svc
                .get_item_charged_cycle_count(&mut self.sol.reuse.cseq_cache, &self.sol.u_data, self.uid)
            {
                Some(InfCount::Count(count)) => Some(count),
                _ => None,
            };
        self.sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_spool_cycle_count(&mut self) -> Option<ItemSpoolInfo> {
        self.sol.svc.get_effect_spool_cycle_count(&self.sol.u_data, self.uid)
    }
}
impl<'s> ItemCommon for Module<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Module<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
}

fn get_u_module(sol: &SolCore, module_uid: UItemId) -> &UModule {
    sol.u_data.items.get(module_uid).dc_module().unwrap()
}
