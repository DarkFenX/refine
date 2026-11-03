use crate::{
    ItemId, RangedProj,
    err::{ProjGetError, basic::ProjFoundError},
    sol::CoreInner,
    ud::UItemId,
};

impl CoreInner {
    pub(in crate::api::item) fn internal_get_ranged_proj(
        &mut self,
        projector_uid: UItemId,
        projectee_id: &ItemId,
    ) -> Result<RangedProj<'_>, ProjGetError> {
        let projectee_uid = self.u_data.items.int_id_by_ext_id_err(projectee_id)?;
        // Unwrapping projections because this method is supposed to be used only with items which
        // have projection container defined on them
        match self
            .u_data
            .items
            .get(projector_uid)
            .get_projs()
            .unwrap()
            .contains(&projectee_uid)
        {
            true => Ok(RangedProj::new(self, projector_uid, projectee_uid)),
            false => Err(ProjFoundError {
                projector_item_id: self.u_data.items.ext_id_by_int_id(projector_uid),
                projectee_item_id: *projectee_id,
            }
            .into()),
        }
    }
}
