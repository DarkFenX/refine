#[derive(Copy, Clone, Eq, PartialEq, Debug, derive_more::Display, derive_more::FromStr)]
#[display(rename_all = "snake_case")]
#[from_str(rename_all = "snake_case")]
pub enum ModRack {
    High,
    Mid,
    Low,
}
