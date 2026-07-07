use super::shared::get_fit_rack_mut;
use crate::{
    api::{ModuleMut, MvMode},
    num::Index,
    sol::SolarSystem,
    ud::{UItemId, UItemVecShiftDir},
};

impl SolarSystem {
    pub(in crate::api) fn internal_move_module(&mut self, module_uid: UItemId, pos_mode: MvMode) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let init_pos = u_module.get_pos();
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, u_module.get_fit_uid(), u_module.get_rack());
        match pos_mode {
            MvMode::Shift(tgt_pos) => {
                if init_pos == tgt_pos {
                    return;
                }
                u_module.set_pos(tgt_pos);
                if let Some((shift_item_ids, shift_dir)) = u_fit_rack.shift(init_pos, tgt_pos) {
                    for shift_item_id in shift_item_ids {
                        let shift_u_module = self.u_data.items.get_mut(shift_item_id).dc_module_mut().unwrap();
                        shift_u_module.set_pos(match shift_dir {
                            UItemVecShiftDir::Left => shift_u_module.get_pos() - Index::ONE,
                            UItemVecShiftDir::Right => shift_u_module.get_pos() + Index::ONE,
                        })
                    }
                }
            }
            MvMode::Swap(tgt_pos) => {
                if init_pos == tgt_pos {
                    return;
                }
                u_module.set_pos(tgt_pos);
                if let Some(tgt_module_uid) = u_fit_rack.swap(init_pos, tgt_pos) {
                    let tgt_u_module = self.u_data.items.get_mut(tgt_module_uid).dc_module_mut().unwrap();
                    tgt_u_module.set_pos(init_pos);
                }
            }
        }
    }
}

impl<'a> ModuleMut<'a> {
    pub fn move_(&mut self, pos_mode: MvMode) {
        self.sol.internal_move_module(self.uid, pos_mode)
    }
}
