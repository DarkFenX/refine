/// Defines what kind of buff effect is providing.
///
/// Depending on buff type, effect chooses items the buff is applied to.
pub enum ABuffType {
    /// Directly affects all items the effect is applied to.
    Everything,
    /// Affects only ships in the same fleet as buff carrier.
    FleetShips,
}
