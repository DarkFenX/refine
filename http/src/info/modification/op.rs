#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModOp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostDiv,
    PostPerc,
    PostAssign,
    ExtraMul,
    Limit,
}
impl From<&rc::SsModOpInfo> for HModOp {
    fn from(core_op: &rc::SsModOpInfo) -> Self {
        match core_op {
            rc::SsModOpInfo::PreAssign => Self::PreAssign,
            rc::SsModOpInfo::PreMul => Self::PreMul,
            rc::SsModOpInfo::PreDiv => Self::PreDiv,
            rc::SsModOpInfo::Add => Self::Add,
            rc::SsModOpInfo::Sub => Self::Sub,
            rc::SsModOpInfo::PostMul => Self::PostMul,
            rc::SsModOpInfo::PostDiv => Self::PostDiv,
            rc::SsModOpInfo::PostPerc => Self::PostPerc,
            rc::SsModOpInfo::PostAssign => Self::PostAssign,
            rc::SsModOpInfo::ExtraMul => Self::ExtraMul,
            rc::SsModOpInfo::MaxLimit => Self::Limit,
        }
    }
}
