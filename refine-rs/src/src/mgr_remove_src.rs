use super::{err::AliasFoundError, mgr::SrcMgr};

impl SrcMgr {
    #[tracing::instrument(name = "srcm-rm", level = "trace", skip_all)]
    pub async fn remove(&self, alias: &str) -> Result<(), RemoveSrcError> {
        tracing::debug!("removing source with alias \"{alias}\"");
        self.alias_src_map
            .write()
            .await
            .remove(alias)
            .ok_or_else(|| AliasFoundError {
                src_alias: alias.to_string(),
            })?;
        let default_alias = self.default_alias.read().await.clone();
        match default_alias {
            Some(a) if a == alias => *self.default_alias.write().await = None,
            _ => (),
        };
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveSrcError {
    #[error("unable to remove source: {0}")]
    SrcNotFound(#[from] AliasFoundError),
}
