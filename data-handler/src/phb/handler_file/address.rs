use std::path::{Path, PathBuf};

pub(super) struct Address {
    folder: &'static str,
    file: &'static str,
}
impl Address {
    pub(super) fn new(folder: &'static str, file: &'static str) -> Self {
        Self { folder, file }
    }
    pub(super) fn get_full_path(&self, base: &Path) -> PathBuf {
        base.join(self.get_part_path())
    }
    pub(super) fn get_part_path(&self) -> PathBuf {
        PathBuf::from(self.folder).join(format!("{}.json", self.file))
    }
    pub(super) fn get_part_str(&self) -> String {
        self.get_part_path().into_string().unwrap_or_else(|_| "<unable to decode path>".to_owned())
    }
}
