use crate::{
    api::{RangedProj, RangedProjMut},
    def::ItemId,
    err::basic::{ItemFoundError, ProjFoundError},
    sol::SolarSystem,
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_ranged_proj(
        &self,
        projector_key: UItemKey,
        projectee_item_id: &ItemId,
    ) -> Result<RangedProj<'_>, GetRangedProjError> {
        let projectee_key = self.internal_get_ranged_projectee_key(projector_key, projectee_item_id)?;
        Ok(RangedProj::new(self, projector_key, projectee_key))
    }
    pub(in crate::api) fn internal_get_ranged_proj_mut(
        &mut self,
        projector_key: UItemKey,
        projectee_item_id: &ItemId,
    ) -> Result<RangedProjMut<'_>, GetRangedProjError> {
        let projectee_key = self.internal_get_ranged_projectee_key(projector_key, projectee_item_id)?;
        Ok(RangedProjMut::new(self, projector_key, projectee_key))
    }
    fn internal_get_ranged_projectee_key(
        &self,
        projector_key: UItemKey,
        projectee_item_id: &ItemId,
    ) -> Result<UItemKey, GetRangedProjError> {
        let projectee_key = self.u_data.items.key_by_id_err(projectee_item_id)?;
        match self
            .u_data
            .items
            .get(projector_key)
            .get_projs()
            .unwrap()
            .contains(&projectee_key)
        {
            true => Ok(projectee_key),
            false => Err(ProjFoundError {
                projector_item_id: self.u_data.items.id_by_key(projector_key),
                projectee_item_id: *projectee_item_id,
            }
            .into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRangedProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
