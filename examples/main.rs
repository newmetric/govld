use std::env;

fn main() {
    let mut cwd_fix = env::current_dir().unwrap().join(file!());
    cwd_fix.pop();
    cwd_fix.push("pkg_main");

    govld::run::do_run(
        cwd_fix.to_str().unwrap(),
        govld::run::Args{
            vendor_dir: "vendor".to_string(),
            force: true,
            patch_manifest_files: vec!["../patch.yaml".to_string()],
        }
    );
}