// https://stackoverflow.com/questions/74841526/why-does-stditerpeekablepeek-mutably-borrow-the-self-argument

pub(crate) struct PrefetchPeekable<I: Iterator> {
    iter: I,
    next_item: Option<I::Item>,
}
impl<I: Iterator> PrefetchPeekable<I> {
    pub(crate) fn new(mut iter: I) -> Self {
        let next_item = iter.next();
        Self { iter, next_item }
    }
    pub(crate) fn peek(&self) -> Option<&I::Item> {
        self.next_item.as_ref()
    }
}
impl<I: Iterator> Iterator for PrefetchPeekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::replace(&mut self.next_item, self.iter.next())
    }
}
