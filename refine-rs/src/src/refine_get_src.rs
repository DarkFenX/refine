use crate::{
    refine::Refine,
    src::{Src, SrcAlias},
};

impl Refine {
    pub async fn get_src(&mut self, alias: Option<SrcAlias>) -> Result<Src<'_>, GetSrcError> {
        let alias = match alias {
            Some(alias) => alias,
            None => match self.default_src_alias.read().await.as_ref() {
                Some(alias) => alias.clone(),
                None => return Err(GetSrcError::DefaultNotDefined),
            },
        };
        let core_src = match self.core_src_map.read().await.get(&alias) {
            Some(core_src) => core_src.clone(),
            None => return Err(GetSrcError::SrcNotFound(alias)),
        };
        Ok(Src::new(self, alias, core_src))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSrcError {
    #[error("alias \"{0}\" not found")]
    SrcNotFound(SrcAlias),
    #[error("default is not defined")]
    DefaultNotDefined,
}
