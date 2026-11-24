use crate::src::{SrcInfoEnumResp, SrcItemInfosCmd, SrcMutatedItemInfosCmd};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SrcInfoEnumCmd {
    ItemInfos(SrcItemInfosCmd),
    MutatedItemInfos(SrcMutatedItemInfosCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfosCmd {
    pub const fn into_src_info(self) -> SrcInfoEnumCmd {
        SrcInfoEnumCmd::ItemInfos(self)
    }
}
impl SrcMutatedItemInfosCmd {
    pub const fn into_src_info(self) -> SrcInfoEnumCmd {
        SrcInfoEnumCmd::MutatedItemInfos(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcInfoEnumCmd {
    pub(crate) fn execute(self, core_src: &rc::Src) -> SrcInfoEnumResp {
        match self {
            Self::ItemInfos(cmd) => SrcInfoEnumResp::ItemInfos(cmd.execute(core_src)),
            Self::MutatedItemInfos(cmd) => SrcInfoEnumResp::MutatedItemInfos(cmd.execute(core_src)),
        }
    }
}
