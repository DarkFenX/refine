#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, derive_more::Display)]
#[display(rename_all = "snake_case")]
pub enum ModRack {
    High,
    Mid,
    Low,
}
