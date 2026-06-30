use crate::cacher_json::data::AdaptedConv;

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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for COp {
    type AEntity = rc::ad::AOp;

    fn from_adapted(a_op: &Self::AEntity) -> Self {
        match a_op {
            Self::AEntity::PreAssign => Self::PreAssign,
            Self::AEntity::PreMul => Self::PreMul,
            Self::AEntity::PreDiv => Self::PreDiv,
            Self::AEntity::Add => Self::Add,
            Self::AEntity::Sub => Self::Sub,
            Self::AEntity::PostMul => Self::PostMul,
            Self::AEntity::PostMulImmune => Self::PostMulImmune,
            Self::AEntity::PostDiv => Self::PostDiv,
            Self::AEntity::PostPerc => Self::PostPerc,
            Self::AEntity::PostPercImmune => Self::PostPercImmune,
            Self::AEntity::PostAssign => Self::PostAssign,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::PreAssign => Self::AEntity::PreAssign,
            Self::PreMul => Self::AEntity::PreMul,
            Self::PreDiv => Self::AEntity::PreDiv,
            Self::Add => Self::AEntity::Add,
            Self::Sub => Self::AEntity::Sub,
            Self::PostMul => Self::AEntity::PostMul,
            Self::PostMulImmune => Self::AEntity::PostMulImmune,
            Self::PostDiv => Self::AEntity::PostDiv,
            Self::PostPerc => Self::AEntity::PostPerc,
            Self::PostPercImmune => Self::AEntity::PostPercImmune,
            Self::PostAssign => Self::AEntity::PostAssign,
        }
    }
}
