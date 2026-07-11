#[derive(Copy, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SrcInfoMode {
    Partial,
    #[default]
    Full,
}
