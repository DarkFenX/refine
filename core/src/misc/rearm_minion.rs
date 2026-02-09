/// Controls when fighters are recalled for rearming/refueling.
#[derive(Copy, Clone)]
pub enum RearmMinion {
    /// No rearm - fighter is kept out even when some of its abilities are out of charges.
    Disabled,
    /// Recall fighter when first of its abilities runs out of charges, after final cycle is
    /// completed.
    OnFirstEmpty,
}
