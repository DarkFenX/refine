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
impl From<&rc::ad::AModOp> for CModOp {
    fn from(mod_op: &rc::ad::AModOp) -> Self {
        match mod_op {
            rc::ad::AModOp::PreAssign => Self::PreAssign,
            rc::ad::AModOp::PreMul => Self::PreMul,
            rc::ad::AModOp::PreDiv => Self::PreDiv,
            rc::ad::AModOp::Add => Self::Add,
            rc::ad::AModOp::Sub => Self::Sub,
            rc::ad::AModOp::PostMul => Self::PostMul,
            rc::ad::AModOp::PostMulImmune => Self::PostMulImmune,
            rc::ad::AModOp::PostDiv => Self::PostDiv,
            rc::ad::AModOp::PostPerc => Self::PostPerc,
            rc::ad::AModOp::PostAssign => Self::PostAssign,
        }
    }
}
impl Into<rc::ad::AModOp> for &CModOp {
    fn into(self) -> rc::ad::AModOp {
        match self {
            CModOp::PreAssign => rc::ad::AModOp::PreAssign,
            CModOp::PreMul => rc::ad::AModOp::PreMul,
            CModOp::PreDiv => rc::ad::AModOp::PreDiv,
            CModOp::Add => rc::ad::AModOp::Add,
            CModOp::Sub => rc::ad::AModOp::Sub,
            CModOp::PostMul => rc::ad::AModOp::PostMul,
            CModOp::PostMulImmune => rc::ad::AModOp::PostMulImmune,
            CModOp::PostDiv => rc::ad::AModOp::PostDiv,
            CModOp::PostPerc => rc::ad::AModOp::PostPerc,
            CModOp::PostAssign => rc::ad::AModOp::PostAssign,
        }
    }
}
