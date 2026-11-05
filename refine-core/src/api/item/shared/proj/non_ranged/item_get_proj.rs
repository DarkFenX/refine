use crate::{
    ItemId, Proj,
    err::{ProjGetError, basic::ProjFoundError},
    sol::SolCore,
    ud::UItemId,
};

impl SolCore {
    pub(in crate::api::item) fn internal_get_proj(
        &mut self,
        projector_uid: UItemId,
        projectee_item_id: &ItemId,
    ) -> Result<Proj<'_>, ProjGetError> {
        let projectee_uid = self.u_data.items.int_id_by_ext_id_err(projectee_item_id)?;
        match self
            .u_data
            .items
            .get(projector_uid)
            .get_projs()
            .unwrap()
            .contains(&projectee_uid)
        {
            true => Ok(Proj::new(self, projector_uid, projectee_uid)),
            false => Err(ProjFoundError {
                projector_item_id: self.u_data.items.ext_id_by_int_id(projector_uid),
                projectee_item_id: *projectee_item_id,
            }
            .into()),
        }
    }
}
