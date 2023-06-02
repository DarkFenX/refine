#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json::cdt) enum ModOp {
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
impl From<rc::consts::ModOp> for ModOp {
    fn from(value: rc::consts::ModOp) -> Self {
        match value {
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
impl Into<rc::consts::ModOp> for ModOp {
    fn into(self) -> rc::consts::ModOp {
        match self {
            ModOp::PreAssign => rc::consts::ModOp::PreAssign,
            ModOp::PreMul => rc::consts::ModOp::PreMul,
            ModOp::PreDiv => rc::consts::ModOp::PreDiv,
            ModOp::Add => rc::consts::ModOp::Add,
            ModOp::Sub => rc::consts::ModOp::Sub,
            ModOp::PostMul => rc::consts::ModOp::PostMul,
            ModOp::PostDiv => rc::consts::ModOp::PostDiv,
            ModOp::PostPerc => rc::consts::ModOp::PostPerc,
            ModOp::PostAssign => rc::consts::ModOp::PostAssign,
        }
    }
}
