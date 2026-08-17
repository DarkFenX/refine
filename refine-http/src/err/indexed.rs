// Carries extra info about source of a command batch execution failure
#[derive(Debug)]
pub(crate) struct ApiErrorIndexed<E> {
    pub(crate) index: usize,
    pub(crate) error: E,
}
impl<E> ApiErrorIndexed<E> {
    pub(crate) fn new(index: usize, error: E) -> Self {
        Self { index, error }
    }
}
impl<E> std::fmt::Display for ApiErrorIndexed<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}
impl<E> std::error::Error for ApiErrorIndexed<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}
