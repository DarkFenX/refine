use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::{debug, error, info, trace, warn};
use serde_json::{Result, Value};

use super::common::DataHandler;

pub struct PhobosDataHandler {
    base_path: PathBuf,
}

impl PhobosDataHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> PhobosDataHandler {
        PhobosDataHandler {
            base_path: path.into(),
        }
    }
    fn read_file(&self, folder_name: &str, file_name: &str) -> Value {
        let full_path = self
            .base_path
            .join(folder_name)
            .join(format!("{}.json", file_name));
        let mut bytes: Vec<u8> = Vec::new();
        File::open(full_path)
            .unwrap()
            .read_to_end(&mut bytes)
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}

impl DataHandler for PhobosDataHandler {
    fn get_evetypes(&self) {
        self.read_file("fsd_lite", "evetypes");
    }
}
