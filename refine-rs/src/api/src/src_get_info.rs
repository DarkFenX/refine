use crate::src::{Src, SrcInfo, SrcInfoMode};

impl Src<'_> {
    pub async fn get_info(&self, info_mode: SrcInfoMode) -> SrcInfo {
        SrcInfo::from_alias_and_core(
            self.inner.get_alias(),
            self.inner.get_time_created(),
            self.inner.get_core().get_info(),
            info_mode,
        )
    }
}
