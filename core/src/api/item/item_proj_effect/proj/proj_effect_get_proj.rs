use crate::{
    api::{Proj, ProjEffect, ProjEffectMut, ProjMut},
    err::basic::{ItemFoundError, ProjFoundError},
    sol::SolarSystem,
    ud::{ItemId, UItemId},
};

impl<'a> ProjEffect<'a> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_uid = get_projectee_uid(self.sol, self.uid, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_uid))
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_uid = get_projectee_uid(self.sol, self.uid, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_uid))
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, GetProjError> {
        let projectee_uid = get_projectee_uid(self.sol, self.uid, projectee_item_id)?;
        Ok(ProjMut::new(self.sol, self.uid, projectee_uid))
    }
}

fn get_projectee_uid(
    sol: &SolarSystem,
    projector_uid: UItemId,
    projectee_item_id: &ItemId,
) -> Result<UItemId, GetProjError> {
    let projectee_uid = sol.u_data.items.iid_by_xid_err(projectee_item_id)?;
    match sol
        .u_data
        .items
        .get(projector_uid)
        .get_projs()
        .unwrap()
        .contains(&projectee_uid)
    {
        true => Ok(projectee_uid),
        false => Err(ProjFoundError {
            projector_item_id: sol.u_data.items.xid_by_iid(projector_uid),
            projectee_item_id: *projectee_item_id,
        }
        .into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
