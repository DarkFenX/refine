use crate::{
    refine::Refine,
    src::{Src, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "src-rm", level = "trace", skip_all)]
    async fn remove_src(&mut self, alias: SrcAlias) {
        tracing::debug!("removing source with alias \"{alias}\"");
        self.core_src_map.write().await.remove(&alias);
        let mut default_alias = self.default_src_alias.write().await;
        if *default_alias == Some(alias) {
            *default_alias = None;
        }
    }
}

impl Src<'_> {
    pub async fn remove(self) {
        self.refine.remove_src(self.alias).await;
    }
}
