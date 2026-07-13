use crate::{
    refine::Refine,
    src::{Src, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "src-rmv", level = "trace", skip_all)]
    async fn remove_src(&self, alias: &SrcAlias) -> Result<(), RemoveSrcError> {
        tracing::debug!("removing source with alias \"{alias}\"");
        let mut alias_data = self.src_alias_data.write().await;
        let extracted_inner_sol = match alias_data.map.remove(alias) {
            Some(inner_sol) => inner_sol,
            None => return Err(RemoveSrcError::SrcNotFound(alias.clone())),
        };
        if let Some(default_inner_sol) = alias_data.default.as_ref()
            && extracted_inner_sol.ptr_eq(default_inner_sol)
        {
            alias_data.default = None;
        };
        Ok(())
    }
}

impl Src<'_> {
    pub async fn remove(self) -> Result<(), RemoveSrcError> {
        self.refine.remove_src(&self.inner.get_alias()).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveSrcError {
    #[error("alias \"{0}\" not found")]
    SrcNotFound(SrcAlias),
}
