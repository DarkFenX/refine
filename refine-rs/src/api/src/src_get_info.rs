use crate::{
    Src,
    src::{SrcInfo, SrcInfoMode},
};

impl Src<'_> {
    #[tracing::instrument(name = "src-inf", level = "error", skip_all)]
    pub async fn get_info(&self, info_mode: SrcInfoMode) -> SrcInfo {
        let is_default = self.refine.src_alias_data.read().is_default(&self.inner);
        SrcInfo::from_inner(&self.inner, is_default, info_mode)
    }
}
