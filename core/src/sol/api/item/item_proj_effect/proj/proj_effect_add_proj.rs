use crate::{
    err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError},
    sol::{
        ItemId, ItemKey, SolarSystem,
        api::{ProjEffectMut, ProjMut},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_proj_effect_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), AddProjError> {
        // Check projector
        let uad_item = self.uad.items.get(item_key);
        let uad_proj_effect = uad_item.get_proj_effect().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if uad_proj_effect.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: uad_proj_effect.get_item_id(),
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
        // Update services
        SolarSystem::util_add_item_projection(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            uad_item,
            projectee_item_key,
            projectee_uad_item,
            None,
        );
        // Update user data
        let uad_proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        uad_proj_effect.get_projs_mut().add(projectee_item_key, None);
        self.rprojs.reg_projectee(item_key, projectee_item_key);
        Ok(())
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut, AddProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_proj_effect_proj(self.key, projectee_item_key)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
