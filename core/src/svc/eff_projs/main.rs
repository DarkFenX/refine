use crate::{
    misc::EffectSpec,
    ud::{UData, UItemKey, UProjData},
    util::RMap,
};

// Holds info about effect projections
#[derive(Clone)]
pub(crate) struct EffProjs {
    pub(super) proj_datas: RMap<(EffectSpec, UItemKey), UProjData>,
}
impl EffProjs {
    pub(in crate::svc) fn new() -> Self {
        Self {
            proj_datas: RMap::new(),
        }
    }
    // Query methods
    pub(crate) fn get_proj_data(&self, projector_espec: EffectSpec, projectee_key: UItemKey) -> Option<UProjData> {
        self.proj_datas.get(&(projector_espec, projectee_key)).copied()
    }
    pub(crate) fn get_or_make_proj_data(
        &self,
        u_data: &UData,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
    ) -> UProjData {
        match self.proj_datas.get(&(projector_espec, projectee_key)) {
            Some(u_proj_data) => *u_proj_data,
            None => {
                let projector_u_item = u_data.items.get(projector_espec.item_key);
                let projectee_u_item = u_data.items.get(projectee_key);
                UProjData::from_physics_with_radii(
                    projector_u_item.get_carrier_physics(u_data),
                    projectee_u_item.get_carrier_physics(u_data),
                    Some(projector_u_item.get_carrier_radius(u_data)),
                    Some(projectee_u_item.get_carrier_radius(u_data)),
                )
            }
        }
    }
    // Modification methods
    pub(in crate::svc) fn add_proj_data(
        &mut self,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
        proj_data: Option<UProjData>,
    ) {
        if let Some(proj_data) = proj_data {
            self.proj_datas.insert((projector_espec, projectee_key), proj_data);
        }
    }
    pub(in crate::svc) fn change_proj_data(
        &mut self,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
        proj_data: Option<UProjData>,
    ) {
        match proj_data {
            Some(proj_data) => self.proj_datas.insert((projector_espec, projectee_key), proj_data),
            None => self.proj_datas.remove(&(projector_espec, projectee_key)),
        };
    }
    pub(in crate::svc) fn remove_proj_data(&mut self, affector_espec: EffectSpec, affectee_key: UItemKey) {
        self.proj_datas.remove(&(affector_espec, affectee_key));
    }
}
