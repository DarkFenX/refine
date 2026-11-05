use crate::{
    PValue,
    stats::{ItemStats, err::StatItemError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub fn get_stat_drone_control_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_drone_control_range(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_warp(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_warp(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_jump_gate(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_jump_gate(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_jump_wormhole(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_jump_wormhole(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_jump_drive(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_jump_drive(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_dock_station(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_dock_station(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_dock_citadel(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_dock_citadel(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_can_tether(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_can_tether(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
}
