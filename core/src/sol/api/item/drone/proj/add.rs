use crate::{
    err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem, api::DroneMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_drone_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), AddDroneProjError> {
        // Check projector
        let uad_drone = self.uad.items.get(item_key).get_drone().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if uad_drone.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: uad_drone.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_uad_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_uad_item.get_item_id(),
                item_kind: projectee_uad_item.get_name(),
            }
            .into());
        }
        // Update user data
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.get_projs_mut().add(projectee_item_key, range);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services
        self.internal_add_item_key_projection_to_svc(item_key, projectee_item_key, range);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId, range: Option<AttrVal>) -> Result<(), AddDroneProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_drone_proj(self.key, projectee_item_key, range)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddDroneProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
