use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Manifest {
    pub file: String,
    pub patch: Vec<Patch>,
}

impl Manifest {
    pub fn from_path(path: &str) -> Self {
        let file = std::fs::File::open(path).expect("error opening manifest");
        let manifest: Self = serde_yaml::from_reader(file).expect("error parsing manifest");

        manifest
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Patch {
    pub pattern: String,
    pub imports: Option<Vec<ManifestImport>>,
    pub code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ManifestImport {
    pub alias: String,
    pub path: String,
}
