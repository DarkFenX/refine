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
impl From<&rc::SolOpInfo> for HModOp {
    fn from(core_op: &rc::SolOpInfo) -> Self {
        match core_op {
            rc::SolOpInfo::PreAssign => Self::PreAssign,
            rc::SolOpInfo::PreMul => Self::PreMul,
            rc::SolOpInfo::PreDiv => Self::PreDiv,
            rc::SolOpInfo::Add => Self::Add,
            rc::SolOpInfo::Sub => Self::Sub,
            rc::SolOpInfo::PostMul => Self::PostMul,
            rc::SolOpInfo::PostDiv => Self::PostDiv,
            rc::SolOpInfo::PostPerc => Self::PostPerc,
            rc::SolOpInfo::PostAssign => Self::PostAssign,
            rc::SolOpInfo::ExtraMul => Self::ExtraMul,
            rc::SolOpInfo::MaxLimit => Self::Limit,
        }
    }
}
