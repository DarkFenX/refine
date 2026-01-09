/// Controls behavior of items like modules which can work both with and without charges.
#[derive(Copy, Clone)]
pub enum ReloadOptionals {
    /// If an item is charged, it will reload upon running out of charges.
    Enabled,
    /// Keep cycling after running out of charges.
    Disabled,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<bool> for ReloadOptionals {
    fn from(value: bool) -> Self {
        match value {
            true => ReloadOptionals::Enabled,
            false => ReloadOptionals::Disabled,
        }
    }
}
