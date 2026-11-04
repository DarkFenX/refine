use crate::{
    Charge, Count, FitId, Index, ItemCommon, ItemSpoolInfo, ModRack, ModuleState, OptionalReload, Spool,
    api::{ItemSealed, active_stat_prepare},
    misc::InfCount,
    sol::CoreInner,
    stats::{StatItemChargeOptions, StatItemStateOptions},
    svc::cycle::CseqMap,
    ud::{UItemId, UModule},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Module<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Module<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_module(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> ModuleState {
        get_u_module(self.core, self.uid).get_module_state()
    }
    pub fn get_rack(&self) -> ModRack {
        get_u_module(self.core, self.uid).get_rack()
    }
    pub fn get_pos(&self) -> Index {
        get_u_module(self.core, self.uid).get_pos()
    }
    pub fn get_charge(&mut self) -> Option<Charge<'_>> {
        get_u_module(self.core, self.uid)
            .get_charge_uid()
            .map(|charge_uid| Charge::new(self.core, charge_uid))
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_u_module(self.core, self.uid).get_charge_count(&self.core.u_data)
    }
    pub fn get_spool(&self) -> Spool {
        self.core.u_data.get_item_spool(self.uid, None)
    }
    pub fn get_spool_override(&self) -> Option<Spool> {
        get_u_module(self.core, self.uid).get_spool_override()
    }
    pub fn get_optional_reload(&self) -> OptionalReload {
        self.core.u_data.get_item_optional_reload(self.uid, None)
    }
    pub fn get_optional_reload_override(&self) -> Option<OptionalReload> {
        get_u_module(self.core, self.uid).get_optional_reload_override()
    }
    pub fn get_charged_cycle_count(&mut self) -> Option<Count> {
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, StatItemStateOptions::Switch);
        let result = match self
            .core
            .svc
            .get_item_charged_cycle_count(&mut CseqMap::new(), &self.core.u_data, self.uid)
        {
            Some(InfCount::Count(count)) => Some(count),
            _ => None,
        };
        self.core.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_spool_cycle_count(&mut self) -> Option<ItemSpoolInfo> {
        self.core.svc.get_effect_spool_cycle_count(&self.core.u_data, self.uid)
    }
}
impl<'s> ItemCommon for Module<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Module<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_core(&self) -> &CoreInner {
        self.core
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        self.core
    }
}

fn get_u_module(core: &CoreInner, module_uid: UItemId) -> &UModule {
    core.u_data.items.get(module_uid).dc_module().unwrap()
}
