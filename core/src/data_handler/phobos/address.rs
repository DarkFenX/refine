use std::path::PathBuf;

pub(super) struct PhobosAddress {
    folder: &'static str,
    file: &'static str,
}

impl PhobosAddress {
    pub(super) fn new(folder: &'static str, file: &'static str) -> PhobosAddress {
        PhobosAddress { folder, file }
    }
    pub(super) fn get_full_path(&self, base: &PathBuf) -> PathBuf {
        base.join(self.get_part_path())
    }
    pub(super) fn get_part_path(&self) -> PathBuf {
        PathBuf::from(self.folder).join(format!("{}.json", self.file))
    }
    pub(super) fn get_full_str(&self, base: &PathBuf) -> String {
        PhobosAddress::path_to_str(&self.get_full_path(base))
    }
    fn path_to_str(path: &PathBuf) -> String {
        match path.to_str() {
            Some(s) => s.to_owned(),
            None => "<unable to decode path>".to_owned(),
        }
    }
}
