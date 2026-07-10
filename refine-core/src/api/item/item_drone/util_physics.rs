use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_drone_physics(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        drone_uid: UItemId,
    ) {
        let u_drone = u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let u_physics = *u_drone.get_physics();
        // Handle outgoing projections
        if !u_drone.get_projs_mut().is_empty() {
            for u_proj_data in u_drone.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_physics(u_physics);
            }
            let u_drone = u_data.items.get(drone_uid).dc_drone().unwrap();
            for (projectee_uid, u_proj_data) in u_drone.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(u_data, svc, drone_uid, projectee_uid, Some(u_proj_data));
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_physics_for_incoming(u_data, rev_projs, svc, drone_uid, u_physics);
    }
}
