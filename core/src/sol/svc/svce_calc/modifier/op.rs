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
impl From<&ad::AOp> for SolOp {
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
            ad::AOp::PostAssign => Self::PostAssign,
        }
    }
}
