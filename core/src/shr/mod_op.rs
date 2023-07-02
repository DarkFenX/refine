/// Defines what kind of operation will be applied to a target attribute.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ModOp {
    /// Assigns modification value to the target item attribute before all other operations are
    /// applied.
    PreAssign,
    /// Early multiplication.
    PreMul,
    /// Early division.
    PreDiv,
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Late multiplication.
    PostMul,
    /// Late division.
    PostDiv,
    /// Late percent-alike modification, e.g. 2 + 20% = 2.4.
    PostPerc,
    /// The same as forcing attribute to modification value. When there is at least one such
    /// modification, all other modification operations are ignored.
    PostAssign,
}
