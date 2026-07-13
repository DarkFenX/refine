#[derive(Default)]
pub(crate) enum TriStateField<T> {
    Value(T),
    None,
    #[default]
    Absent,
}
impl<T> TriStateField<T> {
    pub(crate) fn as_ref(&self) -> TriStateField<&T> {
        match *self {
            Self::Value(ref x) => TriStateField::Value(x),
            Self::None => TriStateField::None,
            Self::Absent => TriStateField::Absent,
        }
    }
    pub(crate) fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> From<Option<T>> for TriStateField<T> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(inner) => Self::Value(inner),
            None => Self::None,
        }
    }
}
impl<T, E> From<Result<T, E>> for TriStateField<T> {
    fn from(val: Result<T, E>) -> Self {
        match val {
            Ok(inner) => Self::Value(inner),
            Err(_) => Self::None,
        }
    }
}

impl<T> From<Option<T>> for TriStateField<Vec<T>> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(inner) => Self::Value(vec![inner]),
            None => Self::None,
        }
    }
}
impl<T, E> From<Result<T, E>> for TriStateField<Vec<T>> {
    fn from(val: Result<T, E>) -> Self {
        match val {
            Ok(inner) => Self::Value(vec![inner]),
            Err(_) => Self::None,
        }
    }
}
