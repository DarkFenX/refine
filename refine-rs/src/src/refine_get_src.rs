use crate::{
    refine::Refine,
    src::{Src, SrcAlias, SrcInnerGuarded},
};

impl Refine {
    // TODO: consider splitting into 2 methods, and making alias parameter generic for convenience
    pub async fn get_src(&self, alias: Option<&SrcAlias>) -> Result<Src<'_>, GetSrcError> {
        let inner_src = self.internal_get_src(alias).await?;
        Ok(Src::new(self, inner_src))
    }
    pub(crate) async fn internal_get_src(&self, alias: Option<&SrcAlias>) -> Result<SrcInnerGuarded, GetSrcError> {
        let alias_data = self.src_alias_data.read().await;
        let alias = match alias {
            Some(alias) => alias,
            None => {
                return match alias_data.default.as_ref() {
                    Some(inner_src) => Ok(inner_src.clone()),
                    None => Err(GetSrcError::DefaultNotDefined),
                };
            }
        };
        match alias_data.map.get(alias) {
            Some(inner_src) => Ok(inner_src.clone()),
            None => Err(GetSrcError::SrcNotFound(alias.clone())),
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
