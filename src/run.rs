use crate::fs_buffer::FsBuffer;
use crate::manifest::Manifest;
use crate::try_patch;
use log::{error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct Args {
    #[arg(short, long, default_value = "vendor")]
    pub vendor_dir: String,

    #[arg(short, long, default_value = "false")]
    pub force: bool,

    pub patch_manifest_files: Vec<String>,
}

pub fn do_run(cwd: &str, args: Args) {
    // force info level
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // if clean mode, try to do a clean setup
    safe_check(args.force, cwd, &args.vendor_dir);

    // organise vendor dir
    let vendor_dir = format!("{}/{}", cwd, args.vendor_dir);

    // organise patch files
    let patch_manifest_files = args
        .patch_manifest_files
        .iter()
        .map(|p| {
            let path = PathBuf::new().join(cwd).join(p.clone());
            path.exists().then(|| path.clone()).unwrap_or_else(|| {
                error!(
                    "error getting patch manifest file: {}",
                    path.to_str().unwrap()
                );
                std::process::exit(127);
            })
        })
        .collect::<Vec<_>>();

    info!("vendor dir: {}", &vendor_dir);
    info!("patch manifest files: {:?}", &patch_manifest_files);

    // for each patch manifest file, try to patch
    // define code buf cache to avoid re-reading the same file
    let fsb = &mut FsBuffer::new(vendor_dir);

    // patches is a global buffer for all patches to be made
    let patches: HashMap<String, Vec<String>> = HashMap::new();
    let imports: HashMap<String, Vec<String>> = HashMap::new();
    let safe_ranges: HashMap<String, std::ops::Range<usize>> = HashMap::new();

    // iterate over all manifest files, try patch
    let (fsb, patches, imports, safe_ranges) = patch_manifest_files.iter().fold(
        (fsb, patches, imports, safe_ranges),
        |(fsb, mut patches, mut imports, mut safe_ranges), path| {
            // read manifest
            let manifest_path = std::fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("error opening file: {}", &path.to_str().unwrap()));
            let manifest: Manifest =
                serde_yaml::from_str(manifest_path.as_str()).unwrap_or_else(|e| {
                    panic!(
                        "error parsing manifest file: {} at {}",
                        e,
                        &path.to_str().unwrap()
                    );
                });

            info!("processing {}", &manifest.file);

            // load code from fsb (loads from file if this is the first occurrence)
            match fsb.try_load(manifest.file.to_owned()) {
                // handle if patch target file is not found
                None => {
                    let is_optional = manifest.optional.unwrap_or(false);

                    if is_optional {
                        info!("skipping optional file: {}", &manifest.file);

                        // short circuit here
                        (fsb, patches, imports, safe_ranges)
                    } else {
                        panic!("error loading file: {}", &manifest.file);
                    }
                }
                Some(code) => {
                    // try patching
                    let result = try_patch(code, &manifest);

                    // update code (with __replaced__ modifications)
                    fsb.update(&manifest.file, &result.code);

                    // update imports
                    imports
                        .entry(manifest.file.to_owned())
                        .or_default()
                        .push(result.imports.join("\n"));

                    // update patches
                    patches
                        .entry(manifest.file.to_owned())
                        .or_default()
                        .push(result.patches.join("\n"));

                    // update safe_ranges (for imports)
                    safe_ranges
                        .entry(manifest.file)
                        .or_insert(result.safe_range);

                    // try post processing
                    if manifest.postprocess.is_some() {
                        for post in manifest.postprocess.unwrap() {
                            let is_optional = post.optional.unwrap_or(false);
                            info!("\tpostprocessing {}", &post.file);

                            match fsb.try_load(post.file.to_owned()) {
                                None => {
                                    if !is_optional {
                                        panic!("error loading postprocess file: {}", &post.file);
                                    } else {
                                        info!("skipping optional file: {}", &post.file);
                                    }
                                }
                                Some(f) => {
                                    let result = try_patch(f, &post);

                                    // update code (with __replaced__ modifications)
                                    fsb.update(&post.file, &result.code);

                                    // update imports
                                    imports
                                        .entry(post.file.to_owned())
                                        .or_default()
                                        .push(result.imports.join("\n"));

                                    // update patches
                                    patches
                                        .entry(post.file.to_owned())
                                        .or_default()
                                        .push(result.patches.join("\n"));

                                    // update safe_ranges (for imports)
                                    safe_ranges.entry(post.file).or_insert(result.safe_range);
                                }
                            }
                        }
                    }

                    // fold over...
                    (fsb, patches, imports, safe_ranges)
                }
            }
        },
    );

    // apply imports first
    // imports append import ( ... ) section at the top of the file
    // but after the "package ..." declaration, using safe_range
    for (path, imports_collected) in imports {
        let import_statements = [
            "import (",
            format!("\t{}", imports_collected.join("\n")).as_str(),
            ")",
        ]
        .join("\n");

        let safe_range = safe_ranges
            .get(&path)
            .unwrap_or_else(|| panic!("error getting safe range for path {}", &path));

        fsb.apply_patch_at(&path, &import_statements, safe_range);
    }

    // apply patches to fsb
    for (path, patches) in patches {
        for patch in patches {
            fsb.append_patch(&path, &patch);
        }
    }

    // actually write to file
    fsb.flush();
}

fn safe_check(force: bool, cwd: &str, vendor_dir: &str) {
    // check if vendor dir exists; fail if yes
    if !force {
        PathBuf::from(vendor_dir).exists().then(|| {
            error!("vendor dir already exists: {}", vendor_dir);
            std::process::exit(127);
        });
    }

    // run go mod vendor
    Command::new("go")
        .current_dir(cwd)
        .args(["mod", "vendor"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        // wait for it to end
        .wait_with_output()
        .unwrap();
}
