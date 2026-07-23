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
pub struct SrcAlias(heapless::String<{ SrcAlias::MAX_LEN }>);
impl SrcAlias {
    pub const MAX_LEN: usize = 100;
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

impl SrcAlias {
    /// Prune received string to make it follow source alias rules:
    ///
    /// - capital letters `A-Z` are lowercased to `a-z`;
    /// - all forbidden characters are replaced by underscores `_`;
    /// - all symbols which are forbidden in leading and trailing positions are stripped;
    /// - if result is longer than 100 characters, only first 100 characters are used.
    ///
    /// If result is an empty string (string of 0 length), None is returned.
    pub fn try_pruned(src_alias: &str) -> Result<Self, SrcAliasPruneInitError> {
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
            return Err(SrcAliasPruneInitError);
        }
        if slice.len() > Self::MAX_LEN {
            slice = &slice[..Self::MAX_LEN];
        }
        Ok(SrcAlias(slice.try_into().unwrap()))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("alias is empty after pruning")]
pub struct SrcAliasPruneInitError;

impl SrcAlias {
    /// Do not try to fix passed string, fail alias instantiation with an error if any of checks do
    /// not pass.
    pub fn try_strict(src_alias: &str) -> Result<Self, SrcAliasStrictInitError> {
        if src_alias.is_empty() {
            return Err(SrcAliasStrictInitError::IsEmpty);
        }
        if src_alias.len() > Self::MAX_LEN {
            return Err(SrcAliasStrictInitError::TooLong(src_alias.len()));
        }
        for c in src_alias.chars() {
            match c {
                'a'..='z' | '0'..='9' | '_' | '-' | '.' => (),
                _ => return Err(SrcAliasStrictInitError::InvalidChar(c)),
            }
        }
        let mut char_iter = src_alias.chars();
        if let Some(c @ '_' | c @ '-' | c @ '.') = char_iter.next() {
            return Err(SrcAliasStrictInitError::InvalidFirstChar(c));
        }
        if let Some(c @ '_' | c @ '-' | c @ '.') = char_iter.last() {
            return Err(SrcAliasStrictInitError::InvalidLastChar(c));
        }
        Ok(SrcAlias(src_alias.try_into().unwrap()))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SrcAliasStrictInitError {
    #[error("string is empty")]
    IsEmpty,
    #[error("string has length of {0}, which is longer than max allowed {max}", max = SrcAlias::MAX_LEN)]
    TooLong(usize),
    #[error("invalid char \"{0}\"")]
    InvalidChar(char),
    #[error("invalid first char \"{0}\"")]
    InvalidFirstChar(char),
    #[error("invalid last char \"{0}\"")]
    InvalidLastChar(char),
}
