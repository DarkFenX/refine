use crate::ch::CacheHandler;

#[derive(Debug)]
pub(crate) struct Src {
    pub alias: String,
    pub cache_handler: Box<dyn CacheHandler>,
}
impl Src {
    pub(crate) fn new(alias: String, cache_handler: Box<dyn CacheHandler>) -> Src {
        Src { alias, cache_handler }
    }
}
