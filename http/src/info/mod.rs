pub(crate) use fit::FitInfo;
pub(crate) use item::ItemInfo;
pub(crate) use ss::SolSysInfo;

mod fit;
mod item;
mod ss;

pub(crate) enum SolSysInfoMode {
    IdOnly,
    Full,
}
impl From<String> for SolSysInfoMode {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "id-only" => Self::IdOnly,
            "full" => Self::Full,
            _ => Self::Full,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum FitInfoMode {
    IdOnly,
    Full,
}
impl From<String> for FitInfoMode {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "id-only" => Self::IdOnly,
            "full" => Self::Full,
            _ => Self::Full,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ItemInfoMode {
    IdOnly,
    Basic,
    Full,
}
impl From<String> for ItemInfoMode {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "id-only" => Self::IdOnly,
            "basic" => Self::Basic,
            "full" => Self::Full,
            _ => Self::Basic,
        }
    }
}
