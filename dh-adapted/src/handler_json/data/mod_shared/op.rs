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
    ExtraMul,
}
impl From<&rc::ModOp> for CModOp {
    fn from(mod_op: &rc::ModOp) -> Self {
        match mod_op {
            rc::ModOp::PreAssign => Self::PreAssign,
            rc::ModOp::PreMul => Self::PreMul,
            rc::ModOp::PreDiv => Self::PreDiv,
            rc::ModOp::Add => Self::Add,
            rc::ModOp::Sub => Self::Sub,
            rc::ModOp::PostMul => Self::PostMul,
            rc::ModOp::PostMulImmune => Self::PostMulImmune,
            rc::ModOp::PostDiv => Self::PostDiv,
            rc::ModOp::PostPerc => Self::PostPerc,
            rc::ModOp::PostAssign => Self::PostAssign,
            rc::ModOp::ExtraMul => Self::ExtraMul,
        }
    }
}
impl Into<rc::ModOp> for &CModOp {
    fn into(self) -> rc::ModOp {
        match self {
            CModOp::PreAssign => rc::ModOp::PreAssign,
            CModOp::PreMul => rc::ModOp::PreMul,
            CModOp::PreDiv => rc::ModOp::PreDiv,
            CModOp::Add => rc::ModOp::Add,
            CModOp::Sub => rc::ModOp::Sub,
            CModOp::PostMul => rc::ModOp::PostMul,
            CModOp::PostMulImmune => rc::ModOp::PostMulImmune,
            CModOp::PostDiv => rc::ModOp::PostDiv,
            CModOp::PostPerc => rc::ModOp::PostPerc,
            CModOp::PostAssign => rc::ModOp::PostAssign,
            CModOp::ExtraMul => rc::ModOp::ExtraMul,
        }
    }
}
