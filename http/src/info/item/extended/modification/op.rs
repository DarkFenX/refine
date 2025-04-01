#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModOp {
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
    ExtraMul,
}
impl From<&rc::OpInfo> for HModOp {
    fn from(core_op: &rc::OpInfo) -> Self {
        match core_op {
            rc::OpInfo::BaseAssign => Self::BaseAssign,
            rc::OpInfo::PreAssign => Self::PreAssign,
            rc::OpInfo::PreMul => Self::PreMul,
            rc::OpInfo::PreDiv => Self::PreDiv,
            rc::OpInfo::Add => Self::Add,
            rc::OpInfo::Sub => Self::Sub,
            rc::OpInfo::PostMul => Self::PostMul,
            rc::OpInfo::PostDiv => Self::PostDiv,
            rc::OpInfo::PostPerc => Self::PostPerc,
            rc::OpInfo::PostAssign => Self::PostAssign,
            rc::OpInfo::MinLimit => Self::MinLimit,
            rc::OpInfo::MaxLimit => Self::MaxLimit,
            rc::OpInfo::ExtraMul => Self::ExtraMul,
        }
    }
}
