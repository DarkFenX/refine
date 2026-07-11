use crate::{
    info::{SrcInfo, SrcInfoMode},
    src::Src,
};

impl Src<'_> {
    pub async fn get_info(&self, src_mode: SrcInfoMode) -> SrcInfo {
        SrcInfo::from_core(self.inner.core_src.get_info(), src_mode)
    }
}
