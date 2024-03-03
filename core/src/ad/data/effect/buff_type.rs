/// Defines what kind of buff effect is providing.
pub enum ABuffType {
    /// Directly affects all items the effect is applied to.
    Everything,
    /// Affects only ships in the same fleet as buff carrier.
    FleetShips,
}
