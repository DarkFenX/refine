#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json) enum COp {
    PreAssign,
    PreMul,
    PreDiv,
    Add,
    Sub,
    PostMul,
    PostMulImmune,
    PostDiv,
    PostPerc,
    PostPercImmune,
    PostAssign,
}
impl From<&rc::ad::AOp> for COp {
    fn from(a_op: &rc::ad::AOp) -> Self {
        match a_op {
            rc::ad::AOp::PreAssign => Self::PreAssign,
            rc::ad::AOp::PreMul => Self::PreMul,
            rc::ad::AOp::PreDiv => Self::PreDiv,
            rc::ad::AOp::Add => Self::Add,
            rc::ad::AOp::Sub => Self::Sub,
            rc::ad::AOp::PostMul => Self::PostMul,
            rc::ad::AOp::PostMulImmune => Self::PostMulImmune,
            rc::ad::AOp::PostDiv => Self::PostDiv,
            rc::ad::AOp::PostPerc => Self::PostPerc,
            rc::ad::AOp::PostPercImmune => Self::PostPercImmune,
            rc::ad::AOp::PostAssign => Self::PostAssign,
        }
    }
}
impl From<&COp> for rc::ad::AOp {
    fn from(c_op: &COp) -> Self {
        match c_op {
            COp::PreAssign => Self::PreAssign,
            COp::PreMul => Self::PreMul,
            COp::PreDiv => Self::PreDiv,
            COp::Add => Self::Add,
            COp::Sub => Self::Sub,
            COp::PostMul => Self::PostMul,
            COp::PostMulImmune => Self::PostMulImmune,
            COp::PostDiv => Self::PostDiv,
            COp::PostPerc => Self::PostPerc,
            COp::PostPercImmune => Self::PostPercImmune,
            COp::PostAssign => Self::PostAssign,
        }
    }
}
