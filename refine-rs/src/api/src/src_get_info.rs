use crate::src::{Src, SrcInfo, SrcInfoModes};

impl Src<'_> {
    pub async fn get_info(&self, modes: SrcInfoModes) -> SrcInfo {
        SrcInfo::from_alias_and_core(
            self.inner.get_alias(),
            self.inner.get_time_created(),
            self.inner.get_core().get_info(),
            modes,
        )
    }
}
