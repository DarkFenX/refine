#[cfg_attr(
    feature = "serde-ad",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AEffectAggroDuration {
    Instant,
    Effect,
}
