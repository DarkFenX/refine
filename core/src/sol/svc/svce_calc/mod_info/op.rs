use crate::sol::svc::svce_calc::SolModOp;

/// Defines what kind of operation will be applied to a target attribute.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(PartialEq)]
pub enum SolModOpInfo {
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
impl From<&SolModOp> for SolModOpInfo {
    fn from(mod_op: &SolModOp) -> Self {
        match mod_op {
            SolModOp::PreAssign => Self::PreAssign,
            SolModOp::PreMul => Self::PreMul,
            SolModOp::PreDiv => Self::PreDiv,
            SolModOp::Add => Self::Add,
            SolModOp::Sub => Self::Sub,
            SolModOp::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            SolModOp::PostMulImmune => Self::PostMul,
            SolModOp::PostDiv => Self::PostDiv,
            SolModOp::PostPerc => Self::PostPerc,
            SolModOp::PostAssign => Self::PostAssign,
            SolModOp::ExtraMul => Self::ExtraMul,
        }
    }
}
