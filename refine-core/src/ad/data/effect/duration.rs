#[cfg_attr(
    feature = "serde-ad",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AEffectWeaponsTimerApplication {
    /// Weapons timer is applied once upon use.
    Instant,
    /// Weapons timer is refreshed over whole duration of effect cycle.
    Effect,
}
