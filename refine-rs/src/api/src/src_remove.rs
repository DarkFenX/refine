use crate::{Refine, Src, SrcAlias};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    pub async fn remove(self) -> Result<(), SrcRemoveError> {
        self.refine.remove_src(&self.inner.get_alias())
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
    fn remove_src(&self, alias: &SrcAlias) -> Result<(), SrcRemoveError> {
        tracing::debug!("removing source with alias \"{alias}\"");
        let mut alias_data = self.src_alias_data.write();
        let removed_src = match alias_data.map.remove(alias) {
            Some(inner_src) => inner_src,
            None => return Err(SrcRemoveError::SrcNotFound(*alias)),
        };
        let removed_default = match alias_data.is_default(&removed_src) {
            true => alias_data.default.take(),
            false => None,
        };
        drop(alias_data);
        // Core source is freed with the last of these unless solar systems still use it
        self.tpool.free_standard((removed_src, removed_default));
        Ok(())
    }
}
