#[derive(Clone)]
pub(in crate::sol::svc::vast) enum ValCache<T, U> {
    Todo(T),
    Pass(T),
    Fail(U),
}
impl<T, U> ValCache<T, U> {
    pub(in crate::sol::svc::vast) fn clear(&mut self, pass: T) {
        *self = Self::Todo(pass)
    }
    pub(in crate::sol::svc::vast) fn pass(&mut self, pass: T) {
        *self = Self::Pass(pass)
    }
    pub(in crate::sol::svc::vast) fn fail(&mut self, fail: U) {
        *self = Self::Fail(fail)
    }
}
