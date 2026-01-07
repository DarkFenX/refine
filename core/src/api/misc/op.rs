use crate::{ad::AOp, svc::calc::CalcOp};

/// Defines what kind of operation will be applied to an attribute being modified.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Op {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Op {
    pub(in crate::api) fn from_a_op(a_op: AOp) -> Self {
        match a_op {
            AOp::PreAssign => Self::PreAssign,
            AOp::PreMul => Self::PreMul,
            AOp::PreDiv => Self::PreDiv,
            AOp::Add => Self::Add,
            AOp::Sub => Self::Sub,
            AOp::PostMul => Self::PostMul,
            AOp::PostMulImmune => Self::PostMul,
            AOp::PostDiv => Self::PostDiv,
            AOp::PostPerc => Self::PostPerc,
            AOp::PostPercImmune => Self::PostPerc,
            AOp::PostAssign => Self::PostAssign,
        }
    }
    pub(crate) fn from_calc_op(calc_op: CalcOp) -> Self {
        match calc_op {
            CalcOp::PreAssign => Self::PreAssign,
            CalcOp::PreMul => Self::PreMul,
            CalcOp::PreDiv => Self::PreDiv,
            CalcOp::Add => Self::Add,
            CalcOp::Sub => Self::Sub,
            CalcOp::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            CalcOp::PostMulImmune => Self::PostMul,
            CalcOp::PostDiv => Self::PostDiv,
            CalcOp::PostPerc => Self::PostPerc,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            CalcOp::PostPercImmune => Self::PostPerc,
            CalcOp::PostAssign => Self::PostAssign,
            CalcOp::ExtraAdd => Self::ExtraAdd,
            CalcOp::ExtraMul => Self::ExtraMul,
        }
    }
}
