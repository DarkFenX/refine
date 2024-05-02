#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModOp {
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
}
impl From<&rc::ad::AOp> for CModOp {
    fn from(mod_op: &rc::ad::AOp) -> Self {
        match mod_op {
            rc::ad::AOp::PreAssign => Self::PreAssign,
            rc::ad::AOp::PreMul => Self::PreMul,
            rc::ad::AOp::PreDiv => Self::PreDiv,
            rc::ad::AOp::Add => Self::Add,
            rc::ad::AOp::Sub => Self::Sub,
            rc::ad::AOp::PostMul => Self::PostMul,
            rc::ad::AOp::PostMulImmune => Self::PostMulImmune,
            rc::ad::AOp::PostDiv => Self::PostDiv,
            rc::ad::AOp::PostPerc => Self::PostPerc,
            rc::ad::AOp::PostAssign => Self::PostAssign,
        }
    }
}
impl Into<rc::ad::AOp> for &CModOp {
    fn into(self) -> rc::ad::AOp {
        match self {
            CModOp::PreAssign => rc::ad::AOp::PreAssign,
            CModOp::PreMul => rc::ad::AOp::PreMul,
            CModOp::PreDiv => rc::ad::AOp::PreDiv,
            CModOp::Add => rc::ad::AOp::Add,
            CModOp::Sub => rc::ad::AOp::Sub,
            CModOp::PostMul => rc::ad::AOp::PostMul,
            CModOp::PostMulImmune => rc::ad::AOp::PostMulImmune,
            CModOp::PostDiv => rc::ad::AOp::PostDiv,
            CModOp::PostPerc => rc::ad::AOp::PostPerc,
            CModOp::PostAssign => rc::ad::AOp::PostAssign,
        }
    }
}
