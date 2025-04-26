use crate::{
    err::basic::{ItemFoundError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_ranged_projectee_item_key(
        &self,
        projector_item_key: ItemKey,
        projectee_item_id: &ItemId,
    ) -> Result<ItemKey, GetRangedProjError> {
        let projectee_item_key = self.uad.items.key_by_id_err(projectee_item_id)?;
        match self
            .uad
            .items
            .get(projector_item_key)
            .get_projs()
            .unwrap()
            .contains(&projectee_item_key)
        {
            true => Ok(projectee_item_key),
            false => Err(ProjFoundError {
                projector_item_id: self.uad.items.id_by_key(projector_item_key),
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
