use crate::{
    api::{RangedProj, RangedProjMut},
    err::basic::{ItemFoundError, ProjFoundError},
    sol::SolarSystem,
    ud::{ItemId, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_ranged_proj(
        &self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<RangedProj<'_>, GetRangedProjError> {
        let projectee_uid = self.internal_get_ranged_projectee_uid(projector_uid, projectee_item_id)?;
        Ok(RangedProj::new(self, projector_uid, projectee_uid))
    }
    pub(in crate::api) fn internal_get_ranged_proj_mut(
        &mut self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<RangedProjMut<'_>, GetRangedProjError> {
        let projectee_uid = self.internal_get_ranged_projectee_uid(projector_uid, projectee_item_id)?;
        Ok(RangedProjMut::new(self, projector_uid, projectee_uid))
    }
    fn internal_get_ranged_projectee_uid(
        &self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<UItemId, GetRangedProjError> {
        let projectee_uid = self.u_data.items.iid_by_xid_err(projectee_item_id)?;
        match self
            .u_data
            .items
            .get(projector_uid)
            .get_projs()
            .unwrap()
            .contains(&projectee_uid)
        {
            true => Ok(projectee_uid),
            false => Err(ProjFoundError {
                projector_item_id: self.u_data.items.xid_by_iid(projector_uid),
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
