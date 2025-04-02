use crate::ad;

/// Defines what kind of operation will be applied to an attribute being modified.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OpInfo {
    /// Uses value of other attribute as a base value. Works same way as PreAssign, but is
    /// calculated earlier, thus gets overridden by any PreAssign. Used only in a few niche
    /// scenarios, like transferring security-zone-specific attribute value to a generic one on
    /// structure rigs.
    BaseAssign,
    /// Assigns modification value to the attribute before all other operations are applied.
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
    /// Limits minimum attribute value.
    MinLimit,
    /// Limits maximum attribute value.
    MaxLimit,
    /// Non-dogma addition operator.
    ExtraAdd,
    /// Non-dogma multiplication operator.
    ExtraMul,
}
impl From<ad::AOp> for OpInfo {
    fn from(a_op: ad::AOp) -> Self {
        match a_op {
            ad::AOp::PreAssign => Self::PreAssign,
            ad::AOp::PreMul => Self::PreMul,
            ad::AOp::PreDiv => Self::PreDiv,
            ad::AOp::Add => Self::Add,
            ad::AOp::Sub => Self::Sub,
            ad::AOp::PostMul => Self::PostMul,
            ad::AOp::PostMulImmune => Self::PostMul,
            ad::AOp::PostDiv => Self::PostDiv,
            ad::AOp::PostPerc => Self::PostPerc,
            ad::AOp::PostPercImmune => Self::PostPerc,
            ad::AOp::PostAssign => Self::PostAssign,
        }
    }
}
