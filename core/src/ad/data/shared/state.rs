#[derive(PartialEq, PartialOrd)]
pub enum AState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}
