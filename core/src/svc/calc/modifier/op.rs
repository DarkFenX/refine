use crate::ad;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum CalcOp {
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
    ExtraAdd,
    ExtraMul,
}
impl From<&ad::AOp> for CalcOp {
    fn from(a_op: &ad::AOp) -> Self {
        match a_op {
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
