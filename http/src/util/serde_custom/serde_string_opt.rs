use serde::Serializer;

pub(crate) fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: Serializer,
{
    match value {
        Some(v) => serializer.collect_str(v),
        None => serializer.serialize_none(),
    }
}
