pub(crate) use attr::AttrValInfo;
pub(crate) use fit::FitInfo;
pub(crate) use item::ItemInfo;
pub(crate) use ss::SsInfo;

mod attr;
mod fit;
mod item;
mod ss;

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SsInfoMode {
    IdOnly,
    Full,
}
impl From<Option<SsInfoMode>> for SsInfoMode {
    fn from(value: Option<SsInfoMode>) -> Self {
        match value {
            Some(v) => v,
            None => Self::Full,
        }
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FitInfoMode {
    IdOnly,
    Full,
}
impl From<Option<FitInfoMode>> for FitInfoMode {
    fn from(value: Option<FitInfoMode>) -> Self {
        match value {
            Some(v) => v,
            None => Self::Full,
        }
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ItemInfoMode {
    IdOnly,
    Basic,
    Full,
}
impl From<Option<ItemInfoMode>> for ItemInfoMode {
    fn from(value: Option<ItemInfoMode>) -> Self {
        match value {
            Some(v) => v,
            None => Self::Basic,
        }
    }
}
