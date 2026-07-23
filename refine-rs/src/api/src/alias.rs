const MAX_LEN: usize = 100;

/// Source alias is a string, which can contain:
///
/// - lowercase letters `a-z`;
/// - digits `0-9`;
/// - underscore `_` (except for leading and trailing position);
/// - hyphen `-` (except for leading and trailing position);
/// - period `.` (except for leading and trailing position).
///
/// Length is also limited, max is 100 characters.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct SrcAlias(heapless::String<MAX_LEN>);
impl SrcAlias {
    /// Prune received string to make it follow source alias rules:
    ///
    /// - capital letters `A-Z` are lowercased to `a-z`;
    /// - all forbidden characters are replaced by underscores `_`;
    /// - all symbols which are forbidden in leading and trailing positions are stripped;
    /// - if result is longer than 100 characters, only first 100 characters are used.
    ///
    /// If result is an empty string (string of 0 length), None is returned.
    pub fn try_pruned(src_alias: &str) -> Option<Self> {
        let mut string: String = src_alias
            .chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-' | '.' => c,
                _ => '_',
            })
            .collect();
        string.make_ascii_lowercase();
        let mut slice = string.trim_start_matches("_-.").trim_end_matches("_-.");
        if slice.is_empty() {
            return None;
        }
        if slice.len() > MAX_LEN {
            slice = &slice[..MAX_LEN];
        }
        Some(SrcAlias(slice.try_into().unwrap()))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<SrcAlias> for String {
    fn from(alias: SrcAlias) -> String {
        alias.0.to_string()
    }
}
impl From<&SrcAlias> for String {
    fn from(alias: &SrcAlias) -> String {
        alias.0.to_string()
    }
}
