use crate::{
    refine::Refine,
    src::{Src, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "src-rm", level = "trace", skip_all)]
    async fn remove_src(&mut self, alias: &SrcAlias) {
        tracing::debug!("removing source with alias \"{alias}\"");
        let mut alias_data = self.src_alias_data.write().await;
        if let Some(extracted_inner_sol) = alias_data.map.remove(alias)
            && let Some(default_inner_sol) = alias_data.default.as_ref()
            && std::sync::Arc::ptr_eq(&extracted_inner_sol, default_inner_sol)
        {
            alias_data.default = None;
        };
    }
}

impl Src<'_> {
    pub async fn remove(self) {
        self.refine.remove_src(&self.inner.alias).await;
    }
}
