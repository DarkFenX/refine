use crate::ud::UItemId;

pub(in crate::api) trait ItemSealed: Sized {
    fn get_uid(&self) -> UItemId;
}
