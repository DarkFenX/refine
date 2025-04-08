use crate::sol::ItemKey;

#[derive(Debug)]
pub(in crate::sol) struct KeyedItemLoadedError {
    pub(in crate::sol) item_key: ItemKey,
}
impl std::error::Error for KeyedItemLoadedError {}
impl std::fmt::Display for KeyedItemLoadedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item with key {} is not loaded", self.item_key)
    }
}
