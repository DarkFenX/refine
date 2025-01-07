use crate::sol::svc::calc::SolOp;

/// Defines what kind of operation will be applied to an attribute being modified.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(PartialEq)]
pub enum SolOpInfo {
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
    /// Non-dogma multiplication operator.
    ExtraMul,
}
impl From<SolOp> for SolOpInfo {
    fn from(mod_op: SolOp) -> Self {
        match mod_op {
            SolOp::PreAssign => Self::PreAssign,
            SolOp::PreMul => Self::PreMul,
            SolOp::PreDiv => Self::PreDiv,
            SolOp::Add => Self::Add,
            SolOp::Sub => Self::Sub,
            SolOp::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            SolOp::PostMulImmune => Self::PostMul,
            SolOp::PostDiv => Self::PostDiv,
            SolOp::PostPerc => Self::PostPerc,
            SolOp::PostAssign => Self::PostAssign,
            SolOp::ExtraMul => Self::ExtraMul,
        }
    }
}
