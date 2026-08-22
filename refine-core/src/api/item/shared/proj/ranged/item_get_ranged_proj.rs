use crate::{
    ItemId, RangedProj, RangedProjMut, SolarSystem,
    err::{GetProjError, basic::ProjFoundError},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api::item) fn internal_get_ranged_proj(
        &self,
        projector_uid: UItemId,
        projectee_id: &ItemId,
    ) -> Result<RangedProj<'_>, GetProjError> {
        let projectee_uid = get_ranged_projectee_uid(self, projector_uid, projectee_id)?;
        Ok(RangedProj::new(self, projector_uid, projectee_uid))
    }
    pub(in crate::api::item) fn internal_get_ranged_proj_mut(
        &mut self,
        projector_uid: UItemId,
        projectee_id: &ItemId,
    ) -> Result<RangedProjMut<'_>, GetProjError> {
        let projectee_uid = get_ranged_projectee_uid(self, projector_uid, projectee_id)?;
        Ok(RangedProjMut::new(self, projector_uid, projectee_uid))
    }
}

fn get_ranged_projectee_uid(
    sol: &SolarSystem,
    projector_uid: UItemId,
    projectee_id: &ItemId,
) -> Result<UItemId, GetProjError> {
    let projectee_uid = sol.u_data.items.int_id_by_ext_id_err(projectee_id)?;
    // Unwrapping projections because this method is supposed to be used only with items which
    // have projection container defined on them
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
            projectee_item_id: *projectee_id,
        }
        .into()),
    }
}
