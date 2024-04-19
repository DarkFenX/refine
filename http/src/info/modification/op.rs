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
impl From<&rc::ModOpInfo> for HModOp {
    fn from(core_op: &rc::ModOpInfo) -> Self {
        match core_op {
            rc::ModOpInfo::PreAssign => Self::PreAssign,
            rc::ModOpInfo::PreMul => Self::PreMul,
            rc::ModOpInfo::PreDiv => Self::PreDiv,
            rc::ModOpInfo::Add => Self::Add,
            rc::ModOpInfo::Sub => Self::Sub,
            rc::ModOpInfo::PostMul => Self::PostMul,
            rc::ModOpInfo::PostDiv => Self::PostDiv,
            rc::ModOpInfo::PostPerc => Self::PostPerc,
            rc::ModOpInfo::PostAssign => Self::PostAssign,
            rc::ModOpInfo::ExtraMul => Self::ExtraMul,
            rc::ModOpInfo::MaxLimit => Self::Limit,
        }
    }
}
