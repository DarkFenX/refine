#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json::data) enum COp {
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
impl COp {
    pub(in crate::cacher_json::data) fn from_adapted(a_op: &rc::ad::AOp) -> Self {
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
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AOp {
        match self {
            Self::PreAssign => rc::ad::AOp::PreAssign,
            Self::PreMul => rc::ad::AOp::PreMul,
            Self::PreDiv => rc::ad::AOp::PreDiv,
            Self::Add => rc::ad::AOp::Add,
            Self::Sub => rc::ad::AOp::Sub,
            Self::PostMul => rc::ad::AOp::PostMul,
            Self::PostMulImmune => rc::ad::AOp::PostMulImmune,
            Self::PostDiv => rc::ad::AOp::PostDiv,
            Self::PostPerc => rc::ad::AOp::PostPerc,
            Self::PostPercImmune => rc::ad::AOp::PostPercImmune,
            Self::PostAssign => rc::ad::AOp::PostAssign,
        }
    }
}
