pub mod fs_buffer;
pub mod manifest;
mod patch;
mod patterns;
pub mod run;

use log::{info, warn};

use crate::manifest::Manifest;
use crate::patterns::try_run;
use patch::parser::Parser;

#[derive(Debug)]
pub struct Result {
    pub module_name: String,
    pub code: String,
    pub imports: Vec<String>,
    pub patches: Vec<String>,

    /// safe_range is the range of the code that is safe to patch.
    /// usually right after the module declaration.
    pub safe_range: std::ops::Range<usize>,
}

/// try_patch
/// code: original source code
/// manifest: patch manifest
pub fn try_patch(code: String, manifest: &Manifest) -> Result {
    // patches is the buffer for the incremental patches.
    // it collects all the patches that will be applied to the code.
    let patches: Vec<String> = Vec::new();

    // imports is the buffer for the additional imports.
    // it collects all the imports that will be appended to the code.
    // further check for duplicate symbols is needed.
    let imports: Vec<String> = Vec::new();

    // find out module name
    let package_parser = Parser::<patterns::module_decl::ModuleDeclPattern>::new(code.as_str());
    let module = package_parser
        .find_first_match()
        .unwrap_or_else(|| panic!("error finding module declaration"));
    let module_name = module.name;

    // find the first safe-point to patch; usually right after the module declaration
    let safe_range = package_parser
        .find_next_line()
        .expect("file does not contain any module declaration");

    info!("patching file: {}", &manifest.file);
    info!("package found: {}", &module_name);

    // for each patch, find the target and patch it
    let (next_code, next_patches, next_imports) = manifest.patch.iter().fold(
        (code, patches, imports),
        |(code, mut patches, mut imports), manifest_patch| {
            let run_result = try_run(
                manifest_patch.pattern.as_str(),
                code.to_owned(),
                manifest_patch.code.to_owned(),
                manifest_patch.patch_type.as_ref(),
            );

            // run may have returned None if no matching pattern is found
            // in this case we just append the patch to the end of the file
            let next_code = match run_result {
                Some(next_code) => next_code,
                None => {
                    warn!(
                        "no matching pattern found for patch: {}; appending",
                        manifest_patch.code
                    );
                    code
                }
            };

            let import_string = match &manifest_patch.imports {
                Some(imports) => imports
                    .iter()
                    .map(|imp| [imp.alias.to_owned(), format!("\"{}\"", imp.path)].join(" "))
                    .collect::<Vec<String>>()
                    .join("\n"),
                None => String::new(),
            };

            imports.push(import_string);
            patches.push(manifest_patch.code.to_owned());

            (next_code, patches, imports)
        },
    );

    // return
    Result {
        module_name,
        safe_range,
        code: next_code,
        imports: next_imports,
        patches: next_patches,
    }
}
