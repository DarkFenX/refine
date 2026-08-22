use crate::{
    Refine,
    src::{Src, SrcAlias},
    svc::SrcInnerGuarded,
};

impl Refine {
    #[tracing::instrument(name = "src-get", level = "trace", skip_all)]
    pub async fn get_src(&self, alias: Option<SrcAlias>) -> Result<Src<'_>, SrcGetError> {
        let inner_src = self.internal_get_src(alias).await?;
        Ok(Src::new(self, inner_src))
    }
    pub(in crate::api) async fn internal_get_src(
        &self,
        alias: Option<SrcAlias>,
    ) -> Result<SrcInnerGuarded, SrcGetError> {
        let alias_data = self.src_alias_data.read().await;
        let alias = match alias {
            Some(alias) => alias,
            None => {
                return match alias_data.default.as_ref() {
                    Some(inner_src) => Ok(inner_src.clone()),
                    None => Err(SrcGetError::DefaultNotDefined),
                };
            }
        };
        match alias_data.map.get(&alias) {
            Some(inner_src) => Ok(inner_src.clone()),
            None => Err(SrcGetError::SrcNotFound(alias)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SrcGetError {
    #[error("alias \"{0}\" not found")]
    SrcNotFound(SrcAlias),
    #[error("default is not defined")]
    DefaultNotDefined,
}
