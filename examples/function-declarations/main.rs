use std::path::PathBuf;
use govld::fs_buffer::FsBuffer;
use govld::manifest::Manifest;
use govld::try_patch;

fn main() {
    // force info level
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let cwd = PathBuf::new().join("./examples/function-declarations/pkg_main");

    // testing function decl
    let manifest_file = std::fs::read_to_string("./examples/function-declarations/patch.yaml").expect("error opening file");
    let manifest: Manifest = serde_yaml::from_str(manifest_file.as_str()).expect("error parsing manifest");

    // sample fsb
    let vendor_dir = cwd.join("vendor");
    let fsb = &mut FsBuffer::new(vendor_dir.to_str().unwrap().to_string());

    let result = try_patch(fsb, manifest).unwrap();
    println!("{:?}", result);

    // usually you would call fsb.flush() here, to save the patched files


    // testing method ecl
    let manifest_file = std::fs::read_to_string("./examples/function-declarations/patch_method.yaml").expect("error opening file");
    let manifest: Manifest = serde_yaml::from_str(manifest_file.as_str()).expect("error parsing manifest");

    // sample fsb
    let vendor_dir = cwd.join("vendor");
    let fsb = &mut FsBuffer::new(vendor_dir.to_str().unwrap().to_string());

    let result = try_patch(fsb, manifest).unwrap();
    println!("{:?}", result);
}