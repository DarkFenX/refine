use crate::src::{SrcItemInfos, SrcMutatedAttrRolls, SrcMutatedItemInfos};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[derive(Clone)]
pub enum SrcInfoEnumResp {
    ItemInfos(SrcItemInfos),
    MutatedItemInfos(SrcMutatedItemInfos),
    MutatedAttrRolls(SrcMutatedAttrRolls),
}
