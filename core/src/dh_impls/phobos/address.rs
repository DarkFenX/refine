use std::path::PathBuf;

pub(super) struct Address {
    folder: &'static str,
    file: &'static str,
}
impl Address {
    pub(super) fn new(folder: &'static str, file: &'static str) -> Address {
        Address { folder, file }
    }
    pub(super) fn get_full_path(&self, base: &PathBuf) -> PathBuf {
        base.join(self.get_part_path())
    }
    pub(super) fn get_part_path(&self) -> PathBuf {
        PathBuf::from(self.folder).join(format!("{}.json", self.file))
    }
    pub(super) fn get_full_str(&self, base: &PathBuf) -> String {
        Address::path_to_str(&self.get_full_path(base))
    }
    pub(super) fn get_part_str(&self) -> String {
        Address::path_to_str(&self.get_part_path())
    }
    fn path_to_str(path: &PathBuf) -> String {
        match path.to_str() {
            Some(s) => s.to_owned(),
            None => "<unable to decode path>".to_owned(),
        }
    }
}
