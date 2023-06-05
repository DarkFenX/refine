use std::path::PathBuf;

pub(in crate::phb::handler_file) struct Address {
    folder: &'static str,
    file: &'static str,
}
impl Address {
    pub(in crate::phb::handler_file) fn new(folder: &'static str, file: &'static str) -> Self {
        Self { folder, file }
    }
    pub(in crate::phb::handler_file) fn get_full_path(&self, base: &PathBuf) -> PathBuf {
        base.join(self.get_part_path())
    }
    pub(in crate::phb::handler_file) fn get_part_path(&self) -> PathBuf {
        PathBuf::from(self.folder).join(format!("{}.json", self.file))
    }
    pub(in crate::phb::handler_file) fn get_part_str(&self) -> String {
        Self::path_to_str(&self.get_part_path())
    }
    fn path_to_str(path: &PathBuf) -> String {
        match path.to_str() {
            Some(s) => s.to_owned(),
            None => "<unable to decode path>".to_owned(),
        }
    }
}
