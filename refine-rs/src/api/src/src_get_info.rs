use crate::src::{Src, SrcInfo, SrcInfoMode};

impl Src<'_> {
    pub async fn get_info(&self, src_mode: SrcInfoMode) -> SrcInfo {
        SrcInfo::from_core(self.inner.get_core().get_info(), src_mode)
    }
}
