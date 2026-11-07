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
        let extracted_inner_sol = match alias_data.map.remove(alias) {
            Some(inner_sol) => inner_sol,
            None => return Err(SrcRemoveError::SrcNotFound(*alias)),
        };
        if alias_data.is_default(&extracted_inner_sol) {
            alias_data.default = None;
        }
        Ok(())
    }
}
