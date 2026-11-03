use super::shared::get_fit_rack_mut;
use crate::{
    Index, Module, RemoveMode,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    pub fn remove(self, pos_mode: RemoveMode) {
        self.sol.internal_remove_module(self.uid, pos_mode)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_module(&mut self, module_uid: UItemId, pos_mode: RemoveMode) {
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let rack = u_module.get_rack();
        let pos = u_module.get_pos();
        let charge_uid = u_module.get_charge_uid();
        // Remove outgoing projections for both module and charge; they keep them, in case they get
        // reattached. Charge projects whatever its module projects
        if !u_module.get_projs().is_empty() {
            CoreInner::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, module_uid);
            if let Some(charge_uid) = charge_uid {
                CoreInner::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, charge_uid);
            }
        }
        // Remove charge from services
        if let Some(charge_uid) = charge_uid {
            CoreInner::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.reuse.eupdates);
        }
        // Remove module from services
        CoreInner::util_remove_module(&mut self.u_data, &mut self.svc, module_uid, &mut self.reuse.eupdates);
        // Update user data - not updating module<->charge references because both are discarded
        // together
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        match pos_mode {
            RemoveMode::Free => u_fit_rack.free(pos),
            RemoveMode::Remove => {
                for shift_module_uid in u_fit_rack.remove(pos) {
                    let shift_u_module = self.u_data.items.get_mut(shift_module_uid).dc_module_mut().unwrap();
                    shift_u_module.set_pos(shift_u_module.get_pos() - Index::ONE)
                }
            }
        }
        if let Some(charge_uid) = charge_uid {
            self.internal_discard_subitem(charge_uid);
        }
        self.internal_discard_item(module_uid, |module_uid| UndoStep::ModuleRemoved(module_uid, pos_mode));
    }
}
