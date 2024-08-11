use crate::util::Error;

pub(in crate::phb::handler_file) trait FromPath<T> {
    fn from_path(err: T, path: &str) -> Self;
}
impl FromPath<std::io::Error> for Error {
    fn from_path(err: std::io::Error, path: &str) -> Self {
        Error::PhbFileSuffixReadFailed(path.to_string(), err.to_string())
    }
}
impl FromPath<serde_json::Error> for Error {
    fn from_path(err: serde_json::Error, path: &str) -> Self {
        if err.is_io() {
            Error::PhbFileSuffixReadFailed(path.to_string(), err.to_string())
        } else {
            Error::PhbFileSuffixParseFailed(path.to_string(), err.to_string())
        }
    }
}
