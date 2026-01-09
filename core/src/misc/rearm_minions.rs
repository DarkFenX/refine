/// Controls behavior of items like fighters which have finite charge on abilities.
#[derive(Copy, Clone)]
pub enum RearmMinions {
    /// Once charges of one of effects run out, an item is recalled for refueling/rearming.
    Enabled,
    /// Item stays out even after charges are out on some of its effects.
    Disabled,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<bool> for RearmMinions {
    fn from(value: bool) -> Self {
        match value {
            true => RearmMinions::Enabled,
            false => RearmMinions::Disabled,
        }
    }
}
