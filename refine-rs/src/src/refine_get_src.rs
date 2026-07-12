use std::sync::Arc;

use crate::{
    refine::Refine,
    src::{Src, SrcAlias, SrcInner},
};

impl Refine {
    pub async fn get_src(&mut self, alias: Option<SrcAlias>) -> Result<Src<'_>, GetSrcError> {
        let inner_src = self.internal_get_inner_src(alias).await?;
        Ok(Src::new(self, inner_src))
    }
    pub(crate) async fn internal_get_inner_src(&self, alias: Option<SrcAlias>) -> Result<SrcInner, GetSrcError> {
        let alias_data = self.src_alias_data.read().await;
        let alias = match alias {
            Some(alias) => alias,
            None => match alias_data.default.as_ref() {
                Some(alias) => alias.clone(),
                None => return Err(GetSrcError::DefaultNotDefined),
            },
        };
        match alias_data.map.get(&alias) {
            Some(core_src) => Ok(SrcInner::new(alias, core_src.clone())),
            None => Err(GetSrcError::SrcNotFound(alias)),
        }
    }
    pub(crate) async fn internal_get_core_src(&self, alias: Option<SrcAlias>) -> Result<Arc<rc::Src>, GetSrcError> {
        let alias_data = self.src_alias_data.read().await;
        let alias = match alias {
            Some(alias) => alias,
            None => match alias_data.default.as_ref() {
                Some(alias) => alias.clone(),
                None => return Err(GetSrcError::DefaultNotDefined),
            },
        };
        match alias_data.map.get(&alias) {
            Some(core_src) => Ok(core_src.clone()),
            None => Err(GetSrcError::SrcNotFound(alias)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSrcError {
    #[error("alias \"{0}\" not found")]
    SrcNotFound(SrcAlias),
    #[error("default is not defined")]
    DefaultNotDefined,
}
