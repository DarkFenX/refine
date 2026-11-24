use crate::src::{SrcItemInfos, SrcMutatedItemInfos};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[derive(Clone)]
pub enum SrcInfoEnumResp {
    ItemInfos(SrcItemInfos),
    MutatedItemInfos(SrcMutatedItemInfos),
}
