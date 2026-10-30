use crate::{
    err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
    ud::{UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_projection(
        &mut self,
        projector_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjAddError> {
        let projector_u_item = self.u_data.items.get(projector_uid);
        match projector_u_item {
            UItem::Drone(..) => self.internal_add_drone_proj(projector_uid, projectee_uid),
            UItem::Fighter(..) => self.internal_add_fighter_proj(projector_uid, projectee_uid),
            UItem::Module(..) => self.internal_add_module_proj(projector_uid, projectee_uid),
            UItem::ProjEffect(..) => self.internal_add_proj_effect_proj(projector_uid, projectee_uid),
            _ => unreachable!("unprojectable item kind is used in projection"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProjAddError {
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error(transparent)]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
