/// Structured option.
///
/// Structured options allow configuring something on 2-3 levels, priority descending:
/// - option passed with a request
/// - option set on an item
/// - solar system option
/// Only first two are set with this enum, on-solar system option is set as just T, since it is the
/// final fallback.
#[derive(Copy, Clone)]
pub enum StOption<T> {
    Set(T),
    Inherit,
}
impl<T> StOption<T> {
    pub(crate) fn is_set(&self) -> bool {
        matches!(self, StOption::Set(_))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> From<Option<T>> for StOption<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => StOption::Set(value),
            None => StOption::Inherit,
        }
    }
}
