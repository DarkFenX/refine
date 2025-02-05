/// States which are used on items and effects.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum AState {
    Offline,
    Online,
    Active,
    Overload,
}
