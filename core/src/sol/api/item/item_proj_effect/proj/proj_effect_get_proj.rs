use crate::{
    def::{ItemId, ItemKey},
    err::basic::{ItemFoundError, ProjFoundError},
    sol::{
        SolarSystem,
        api::{Proj, ProjEffect, ProjEffectMut, ProjMut},
    },
};

impl<'a> ProjEffect<'a> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_item_key = get_projectee_item_key(self.sol, self.key, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_item_key))
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<Proj<'_>, GetProjError> {
        let projectee_item_key = get_projectee_item_key(self.sol, self.key, projectee_item_id)?;
        Ok(Proj::new(self.sol, projectee_item_key))
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, GetProjError> {
        let projectee_item_key = get_projectee_item_key(self.sol, self.key, projectee_item_id)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_item_key))
    }
}

fn get_projectee_item_key(
    sol: &SolarSystem,
    projector_item_key: ItemKey,
    projectee_item_id: &ItemId,
) -> Result<ItemKey, GetProjError> {
    let projectee_item_key = sol.uad.items.key_by_id_err(projectee_item_id)?;
    match sol
        .uad
        .items
        .get(projector_item_key)
        .get_projs()
        .unwrap()
        .contains(&projectee_item_key)
    {
        true => Ok(projectee_item_key),
        false => Err(ProjFoundError {
            projector_item_id: sol.uad.items.id_by_key(projector_item_key),
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
