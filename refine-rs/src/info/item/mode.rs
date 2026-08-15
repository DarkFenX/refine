#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum ItemInfoMode {
    Id,
    #[default]
    Partial,
    Full,
}
