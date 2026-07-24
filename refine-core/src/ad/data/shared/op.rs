#[cfg_attr(
    feature = "serde-ad",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AOp {
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
