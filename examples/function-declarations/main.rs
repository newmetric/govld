use std::path::PathBuf;
use govld::manifest::Manifest;
use govld::try_patch;

fn main() {
    let cwd = PathBuf::new().join("./examples/function-declarations/pkg_main/vendor");
    let manifest_file = std::fs::read_to_string("./examples/function-declarations/patch.yaml").expect("error opening file");
    let manifest: Manifest = serde_yaml::from_str(manifest_file.as_str()).expect("error parsing manifest");

    dbg!(&manifest);


    let cwd_str = cwd.to_str().unwrap();
    let result = try_patch(cwd_str, manifest).unwrap();


    println!("{:?}", result);
}