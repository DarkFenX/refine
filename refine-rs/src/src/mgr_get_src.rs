use super::{err::AliasFoundError, mgr::SrcMgr, src::Src};

impl SrcMgr {
    pub async fn get(&self, alias: Option<&str>) -> Result<Src, GetSrcError> {
        match alias {
            Some(a) => self.get_src_by_alias(a).await,
            None => self.get_default_src().await,
        }
    }
    async fn get_src_by_alias(&self, alias: &str) -> Result<Src, GetSrcError> {
        self.alias_src_map.read().await.get(alias).cloned().ok_or_else(|| {
            AliasFoundError {
                src_alias: alias.to_string(),
            }
            .into()
        })
    }
    async fn get_default_src(&self) -> Result<Src, GetSrcError> {
        match self.default_alias.read().await.as_ref() {
            Some(a) => self.get_src_by_alias(a).await,
            None => Err(GetSrcError::DefaultNotDefined),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSrcError {
    #[error("unable to get source: {0}")]
    SrcNotFound(#[from] AliasFoundError),
    #[error("unable to get source: default is not defined")]
    DefaultNotDefined,
}
