use crate::{
    ItemId, Proj, ProjMut, SolarSystem,
    err::{GetProjError, basic::ProjFoundError},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api::item) fn internal_get_proj(
        &self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<Proj<'_>, GetProjError> {
        let projectee_uid = get_projectee_uid(self, projector_uid, projectee_item_id)?;
        Ok(Proj::new(self, projectee_uid))
    }
    pub(in crate::api::item) fn internal_get_proj_mut(
        &mut self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<ProjMut<'_>, GetProjError> {
        let projectee_uid = get_projectee_uid(self, projector_uid, projectee_item_id)?;
        Ok(ProjMut::new(self, projector_uid, projectee_uid))
    }
}

fn get_projectee_uid(
    sol: &SolarSystem,
    projector_uid: UItemId,
    projectee_item_id: &ItemId,
) -> Result<UItemId, GetProjError> {
    let projectee_uid = sol.u_data.items.int_id_by_ext_id_err(projectee_item_id)?;
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
            projector_item_id: sol.u_data.items.ext_id_by_int_id(projector_uid),
            projectee_item_id: *projectee_item_id,
        }
        .into()),
    }
}
