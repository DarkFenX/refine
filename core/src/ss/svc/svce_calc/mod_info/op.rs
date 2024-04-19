use crate::ss::svc::svce_calc::SsModOp;

/// Defines what kind of operation will be applied to a target attribute.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(PartialEq)]
pub enum SsModOpInfo {
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
    /// Limits maximum attribute value.
    MaxLimit,
    /// Non-dogma multiplication operator.
    ExtraMul,
}
impl From<&SsModOp> for SsModOpInfo {
    fn from(mod_op: &SsModOp) -> Self {
        match mod_op {
            SsModOp::PreAssign => Self::PreAssign,
            SsModOp::PreMul => Self::PreMul,
            SsModOp::PreDiv => Self::PreDiv,
            SsModOp::Add => Self::Add,
            SsModOp::Sub => Self::Sub,
            SsModOp::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            SsModOp::PostMulImmune => Self::PostMul,
            SsModOp::PostDiv => Self::PostDiv,
            SsModOp::PostPerc => Self::PostPerc,
            SsModOp::PostAssign => Self::PostAssign,
            SsModOp::ExtraMul => Self::ExtraMul,
        }
    }
}
