use crate::{sol::CoreInner, ud::UItemId};

pub(in crate::api) trait ItemSealed: Sized {
    fn get_uid(&self) -> UItemId;
    fn get_core(&self) -> &CoreInner;
    fn get_core_mut(&mut self) -> &mut CoreInner;
}
