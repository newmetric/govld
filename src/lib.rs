mod patterns;
mod patch;
pub mod manifest;

use std::cell::RefCell;
use patch::parser::Parser;
use crate::manifest::Manifest;
use crate::patterns::Pattern;

#[derive(Debug)]
pub struct Result {
    pub path: String,
    pub module: String,
    pub code: String,
    pub patch: String,
}

pub fn try_patch(vendor_dir: &str, manifest: Manifest) -> Option<Result> {
    // pre-create next code buffer
    let patch: RefCell<String> = RefCell::new(String::new());

    // figure out actual package name
    let code_path = format!("{}/{}", vendor_dir, manifest.file);
    let mut code = std::fs::read_to_string(code_path).expect("error opening file");
    let package_parser = Parser::<patterns::module_decl::ModuleDeclPattern>::new(code.as_str());

    let module = package_parser.find_first_match().expect("error finding match");
    let module_name = module.name;

    // for each patch, find the target and patch it
    for manifest_patch in manifest.patch {
        // parse target first and get pattern
        let target_p = match manifest_patch.pattern.as_str() {
            "function_declaration" => Parser::<patterns::func_decl::FunctionDeclPattern>::new(manifest_patch.patch.as_str()),
            _ => panic!("unknown pattern")
        };
        let target_pattern = target_p.find_first_match().expect("error finding match");

        // parse target and get pattern
        let mut source_p = match manifest_patch.pattern.as_str() {
            "function_declaration" => Parser::<patterns::func_decl::FunctionDeclPattern>::new(code.as_str()),
            _ => panic!("unknown pattern")
        };

        // for each pattern found, patch the original file && keep a separate file
        let next = source_p
            .find_and_patch(|pat| {
                let is_match = pat.is_match(&target_pattern);
                if is_match {
                    patch.borrow_mut().push_str(manifest_patch.patch.as_str());
                    patch.borrow_mut().push('\n');
                }

                is_match
            });

        code = next.unwrap_or_else(|| panic!("no matching pattern or function {} found", &target_pattern.name));
    }

    let patch = patch.borrow().clone();

    // slightly modify manifest.file into {}_patched.go
    let patch_file_path = manifest.file.replace(".go", "_patched.go");

    // return
    Some(Result{
        path: format!("{}/{}", vendor_dir, patch_file_path),
        module: module_name,
        code,
        patch,
    })
}
