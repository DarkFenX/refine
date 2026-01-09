use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::info::item::extended::modification) enum HModOp {
    BaseAssign,
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPerc,
    PostAssign,
    MinLimit,
    MaxLimit,
    ExtraAdd,
    ExtraMul,
}
impl HModOp {
    pub(in crate::info::item::extended::modification) fn from_core(core_op: rc::Op) -> Self {
        match core_op {
            rc::Op::BaseAssign => Self::BaseAssign,
            rc::Op::PreAssign => Self::PreAssign,
            rc::Op::PreMul => Self::PreMul,
            rc::Op::PreDiv => Self::PreDiv,
            rc::Op::Add => Self::Add,
            rc::Op::Sub => Self::Sub,
            rc::Op::PostMul => Self::PostMul,
            rc::Op::PostDiv => Self::PostDiv,
            rc::Op::PostPerc => Self::PostPerc,
            rc::Op::PostAssign => Self::PostAssign,
            rc::Op::MinLimit => Self::MinLimit,
            rc::Op::MaxLimit => Self::MaxLimit,
            rc::Op::ExtraAdd => Self::ExtraAdd,
            rc::Op::ExtraMul => Self::ExtraMul,
        }
    }
}
