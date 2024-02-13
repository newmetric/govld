use std::path::Path;

use serde::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Manifest {
    pub file: String,
    pub patch: Vec<Patch>,

    // optional signifies that the patch is optional
    pub optional: Option<bool>,

    // run this AFTER the patch is applied without errors
    pub postprocess: Option<Vec<Manifest>>,
}

impl Manifest {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let file = std::fs::File::open(path).expect("error opening manifest");
        let manifest: Self = serde_yaml::from_reader(file).expect("error parsing manifest");

        manifest
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PatchType {
    Clone,
    Overwrite,
}

fn patch_type_from_str<'de, D>(deserializer: D) -> Result<Option<PatchType>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    println!("{s:?}");

    let Some(s) = s else {
        return Ok(None);
    };

    // do better hex decoding than this
    match s {
        "clone" => Ok(Some(PatchType::Clone)),
        "overwrite" => Ok(Some(PatchType::Overwrite)),
        _ => Err(serde::de::Error::custom("invalid patch type")),
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Patch {
    pub pattern: String,
    #[serde(deserialize_with = "patch_type_from_str")]
    #[serde(default)]
    pub patch_type: Option<PatchType>,
    pub imports: Option<Vec<ManifestImport>>,
    pub code: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ManifestImport {
    pub alias: String,
    pub path: String,
}
