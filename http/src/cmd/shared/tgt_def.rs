#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HTgtDef {
    Full(HTgtDefFull),
    Short(HTgtDefShort),
    IdOnly(#[serde_as(as = "serde_with::DisplayFromStr")] rc::SolItemId),
}
impl HTgtDef {
    pub(in crate::cmd) fn get_item_id(&self) -> rc::SolItemId {
        match self {
            Self::Full(tgt_def) => tgt_def.item_id,
            Self::Short(tgt_def) => tgt_def.item_id,
            Self::IdOnly(item_id) => *item_id,
        }
    }
    pub(in crate::cmd) fn get_range(&self) -> Option<rc::AttrVal> {
        match self {
            Self::Full(tgt_def) => tgt_def.range,
            Self::Short(_) => None,
            Self::IdOnly(_) => None,
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(in crate::cmd) struct HTgtDefFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    range: Option<rc::AttrVal>,
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(in crate::cmd) struct HTgtDefShort {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
}
