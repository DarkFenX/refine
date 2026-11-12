use crate::{Refine, Src, SrcAlias, svc::SrcInnerGuarded};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    pub async fn remove(self) -> Result<(), SrcRemoveError> {
        self.refine.remove_src(self.inner)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SrcRemoveError {
    #[error("alias \"{0}\" not found")]
    SrcNotFound(SrcAlias),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Refine {
    #[tracing::instrument(name = "src-rmv", level = "error", skip_all)]
    fn remove_src(&self, inner_src: SrcInnerGuarded) -> Result<(), SrcRemoveError> {
        let alias = inner_src.get_alias();
        tracing::debug!("removing source with alias \"{alias}\"");
        let mut alias_data = self.src_alias_data.write();
        // The alias could have been removed and given to another source since the handle was taken
        let removed_src = match alias_data.map.get(&alias) {
            Some(map_src) if map_src.ptr_eq(&inner_src) => alias_data.map.remove(&alias),
            _ => None,
        };
        let removed_default = match removed_src.is_some() && alias_data.is_default(&inner_src) {
            true => alias_data.default.take(),
            false => None,
        };
        drop(alias_data);
        let removed = removed_src.is_some();
        // The handle's reference goes along, so that the core source is freed in the job unless
        // solar systems or other requests still use it
        self.tpool.free_standard((inner_src, removed_src, removed_default));
        match removed {
            true => Ok(()),
            false => Err(SrcRemoveError::SrcNotFound(alias)),
        }
    }
}
