use crate::ch::CacheHandler;

#[derive(Debug)]
pub struct Src<T>
where
    T: CacheHandler,
{
    pub alias: String,
    pub cache_handler: T,
}
impl<T> Src<T>
where
    T: CacheHandler,
{
    pub fn new(alias: String, cache_handler: T) -> Src<T> {
        Src { alias, cache_handler }
    }
}
