#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModOp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPerc,
    PostAssign,
}
impl From<&rc::consts::ModOp> for CModOp {
    fn from(mod_op: &rc::consts::ModOp) -> Self {
        match mod_op {
            rc::consts::ModOp::PreAssign => Self::PreAssign,
            rc::consts::ModOp::PreMul => Self::PreMul,
            rc::consts::ModOp::PreDiv => Self::PreDiv,
            rc::consts::ModOp::Add => Self::Add,
            rc::consts::ModOp::Sub => Self::Sub,
            rc::consts::ModOp::PostMul => Self::PostMul,
            rc::consts::ModOp::PostDiv => Self::PostDiv,
            rc::consts::ModOp::PostPerc => Self::PostPerc,
            rc::consts::ModOp::PostAssign => Self::PostAssign,
        }
    }
}
impl Into<rc::consts::ModOp> for &CModOp {
    fn into(self) -> rc::consts::ModOp {
        match self {
            CModOp::PreAssign => rc::consts::ModOp::PreAssign,
            CModOp::PreMul => rc::consts::ModOp::PreMul,
            CModOp::PreDiv => rc::consts::ModOp::PreDiv,
            CModOp::Add => rc::consts::ModOp::Add,
            CModOp::Sub => rc::consts::ModOp::Sub,
            CModOp::PostMul => rc::consts::ModOp::PostMul,
            CModOp::PostDiv => rc::consts::ModOp::PostDiv,
            CModOp::PostPerc => rc::consts::ModOp::PostPerc,
            CModOp::PostAssign => rc::consts::ModOp::PostAssign,
        }
    }
}
