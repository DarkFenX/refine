use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemReceiveProjError, ProjNotFoundError},
    sol::{
        SolarSystem,
        api::{ProjEffectMut, ProjMut},
    },
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_proj_effect_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_item = self.u_data.items.get(item_key);
        let u_proj_effect = u_item.get_proj_effect().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_proj_effect.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_proj_effect.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_u_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        // Update services
        SolarSystem::util_add_item_projection(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
            None,
        );
        // Update user data
        let u_proj_effect = self.u_data.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        u_proj_effect.get_projs_mut().add(projectee_key, None);
        self.rprojs.reg_projectee(item_key, projectee_key);
        Ok(())
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, AddProjError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_proj_effect_proj(self.key, projectee_key)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_key))
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
