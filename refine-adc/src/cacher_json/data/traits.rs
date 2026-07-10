pub(super) trait AdaptedConv {
    type AEntity;

    fn from_adapted(a_entity: &Self::AEntity) -> Self;
    fn into_adapted(self) -> Self::AEntity;
}
