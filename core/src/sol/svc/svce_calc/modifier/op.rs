use crate::ad;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolOp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostMulImmune,
    PostDiv,
    PostPerc,
    PostAssign,
    ExtraMul,
}
impl From<&ad::AModOp> for SolOp {
    fn from(a_mod_op: &ad::AModOp) -> Self {
        match a_mod_op {
            ad::AModOp::PreAssign => Self::PreAssign,
            ad::AModOp::PreMul => Self::PreMul,
            ad::AModOp::PreDiv => Self::PreDiv,
            ad::AModOp::Add => Self::Add,
            ad::AModOp::Sub => Self::Sub,
            ad::AModOp::PostMul => Self::PostMul,
            ad::AModOp::PostMulImmune => Self::PostMulImmune,
            ad::AModOp::PostDiv => Self::PostDiv,
            ad::AModOp::PostPerc => Self::PostPerc,
            ad::AModOp::PostAssign => Self::PostAssign,
        }
    }
}
