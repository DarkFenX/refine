#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HProjDef {
    Full(HProjDefFull),
    Short(HProjDefShort),
    IdOnly(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ItemId),
}
impl HProjDef {
    pub(in crate::cmd) fn get_item_id(&self) -> rc::ItemId {
        match self {
            Self::Full(proj_def) => proj_def.get_item_id(),
            Self::Short(proj_def) => proj_def.item_id,
            Self::IdOnly(item_id) => *item_id,
        }
    }
    pub(in crate::cmd) fn get_range(&self) -> Option<rc::AttrVal> {
        match self {
            Self::Full(proj_def) => proj_def.get_range(),
            Self::Short(_) => None,
            Self::IdOnly(_) => None,
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(in crate::cmd) struct HProjDefFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    range: Option<rc::AttrVal>,
}
impl HProjDefFull {
    pub(in crate::cmd) fn get_item_id(&self) -> rc::ItemId {
        self.item_id
    }
    pub(in crate::cmd) fn get_range(&self) -> Option<rc::AttrVal> {
        self.range
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(in crate::cmd) struct HProjDefShort {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
}
