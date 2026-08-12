use super::error::ReadParseFailReason;

// Returns first entry accepted by the predicate
pub(in crate::sde) fn first_in_lines<T>(
    mut reader: impl std::io::BufRead,
    mut predicate: impl FnMut(&T) -> bool,
) -> Result<Option<T>, ReadParseFailReason>
where
    T: serde::de::DeserializeOwned,
{
    let mut line = String::new();
    while {
        line.clear();
        reader.read_line(&mut line)?
    } > 0
    {
        // Skip lines which cannot be converted to the target type
        let Ok(value) = serde_json::from_str::<T>(&line) else {
            continue;
        };
        if predicate(&value) {
            return Ok(Some(value));
        }
    }
    Ok(None)
}
