use crate::ch;

#[derive(Debug)]
pub(crate) struct Src {
    pub(crate) alias: String,
    pub(crate) cache_handler: Box<dyn ch::CacheHandler>,
}
impl Src {
    pub(crate) fn new(alias: String, cache_handler: Box<dyn ch::CacheHandler>) -> Src {
        Src { alias, cache_handler }
    }
}
