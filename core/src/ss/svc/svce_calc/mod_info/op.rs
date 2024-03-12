use crate::shr::ModOp;

#[derive(Debug, PartialEq)]
pub enum ModOpInfo {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPerc,
    PostAssign,
    Limit,
    ExtraMul,
}
impl From<&ModOp> for ModOpInfo {
    fn from(mod_op: &ModOp) -> Self {
        match mod_op {
            ModOp::PreAssign => Self::PreAssign,
            ModOp::PreMul => Self::PreMul,
            ModOp::PreDiv => Self::PreDiv,
            ModOp::Add => Self::Add,
            ModOp::Sub => Self::Sub,
            ModOp::PostMul => Self::PostMul,
            // Since info already exposes if modification is penalized or not, we don't need to have
            // this operator to be part of the info
            ModOp::PostMulImmune => Self::PostMul,
            ModOp::PostDiv => Self::PostDiv,
            ModOp::PostPerc => Self::PostPerc,
            ModOp::PostAssign => Self::PostAssign,
            ModOp::ExtraMul => Self::ExtraMul,
        }
    }
}
