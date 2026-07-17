use std::path::{Path, PathBuf};

pub(super) struct Address {
    dir: &'static str,
    file: &'static str,
}
impl Address {
    pub(super) fn new(dir: &'static str, file: &'static str) -> Self {
        Self { dir, file }
    }
    pub(super) fn get_full_path(&self, base: &Path) -> PathBuf {
        base.join(self.get_part_path())
    }
    pub(super) fn get_part_path(&self) -> PathBuf {
        PathBuf::from(self.dir).join(format!("{}.json", self.file))
    }
    pub(super) fn get_part_str(&self) -> String {
        self.get_part_path()
            .into_string()
            .unwrap_or_else(|_| "<unable to decode path>".to_owned())
    }
}
