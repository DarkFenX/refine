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
impl From<&rc::SolModOpInfo> for HModOp {
    fn from(core_op: &rc::SolModOpInfo) -> Self {
        match core_op {
            rc::SolModOpInfo::PreAssign => Self::PreAssign,
            rc::SolModOpInfo::PreMul => Self::PreMul,
            rc::SolModOpInfo::PreDiv => Self::PreDiv,
            rc::SolModOpInfo::Add => Self::Add,
            rc::SolModOpInfo::Sub => Self::Sub,
            rc::SolModOpInfo::PostMul => Self::PostMul,
            rc::SolModOpInfo::PostDiv => Self::PostDiv,
            rc::SolModOpInfo::PostPerc => Self::PostPerc,
            rc::SolModOpInfo::PostAssign => Self::PostAssign,
            rc::SolModOpInfo::ExtraMul => Self::ExtraMul,
            rc::SolModOpInfo::MaxLimit => Self::Limit,
        }
    }
}
