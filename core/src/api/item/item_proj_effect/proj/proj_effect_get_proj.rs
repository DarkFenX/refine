use crate::{
    api::{Proj, ProjEffect, ProjEffectMut, ProjMut},
    def::ItemId,
    err::basic::{ItemFoundError, ProjFoundError},
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> ProjEffect<'a> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_key = get_projectee_key(self.sol, self.key, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_key))
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_key = get_projectee_key(self.sol, self.key, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_key))
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, GetProjError> {
        let projectee_key = get_projectee_key(self.sol, self.key, projectee_item_id)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_key))
    }
}

fn get_projectee_key(
    sol: &SolarSystem,
    projector_key: UItemId,
    projectee_item_id: &ItemId,
) -> Result<UItemId, GetProjError> {
    let projectee_key = sol.u_data.items.int_id_by_ext_id_err(projectee_item_id)?;
    match sol
        .u_data
        .items
        .get(projector_key)
        .get_projs()
        .unwrap()
        .contains(&projectee_key)
    {
        true => Ok(projectee_key),
        false => Err(ProjFoundError {
            projector_item_id: sol.u_data.items.ext_id_by_int_id(projector_key),
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
