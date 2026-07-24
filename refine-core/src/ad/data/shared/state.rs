#[cfg_attr(
    feature = "serde-ad",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum AState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}
