use crate::{ad, sol::OpInfo};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum Op {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostMulImmune,
    PostDiv,
    PostPerc,
    PostPercImmune,
    PostAssign,
    ExtraMul,
}
impl From<&ad::AOp> for Op {
    fn from(a_mod_op: &ad::AOp) -> Self {
        match a_mod_op {
            ad::AOp::PreAssign => Self::PreAssign,
            ad::AOp::PreMul => Self::PreMul,
            ad::AOp::PreDiv => Self::PreDiv,
            ad::AOp::Add => Self::Add,
            ad::AOp::Sub => Self::Sub,
            ad::AOp::PostMul => Self::PostMul,
            ad::AOp::PostMulImmune => Self::PostMulImmune,
            ad::AOp::PostDiv => Self::PostDiv,
            ad::AOp::PostPerc => Self::PostPerc,
            ad::AOp::PostPercImmune => Self::PostPercImmune,
            ad::AOp::PostAssign => Self::PostAssign,
        }
    }
}
impl From<Op> for OpInfo {
    fn from(mod_op: Op) -> Self {
        match mod_op {
            Op::PreAssign => Self::PreAssign,
            Op::PreMul => Self::PreMul,
            Op::PreDiv => Self::PreDiv,
            Op::Add => Self::Add,
            Op::Sub => Self::Sub,
            Op::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            Op::PostMulImmune => Self::PostMul,
            Op::PostDiv => Self::PostDiv,
            Op::PostPerc => Self::PostPerc,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            Op::PostPercImmune => Self::PostPerc,
            Op::PostAssign => Self::PostAssign,
            Op::ExtraMul => Self::ExtraMul,
        }
    }
}
