mod patterns;
mod patch;
pub mod manifest;
pub mod fs_buffer;

use log::{info, warn};

use patch::parser::Parser;
use crate::manifest::Manifest;
use crate::patterns::Pattern;

#[derive(Debug)]
pub struct Result {
    pub module_name: String,
    pub code: String,
    pub imports: Vec<String>,
    pub patches: Vec<String>,

    // safe_range is the range of the code that is safe to patch.
    // usually right after the module declaration.
    pub safe_range: std::ops::Range<usize>,
}

// try_patch
// code: original source code
// manifest: patch manifest
pub fn try_patch(code: String, manifest: &Manifest) -> Option<Result> {
    // patches is the buffer for the incremental patches.
    // it collects all the patches that will be applied to the code.
    let patches: Vec<String> = Vec::new();

    // imports is the buffer for the additional imports.
    // it collects all the imports that will be appended to the code.
    // further check for duplicate symbols is needed.
    let imports: Vec<String> = Vec::new();

    // find out module name
    let package_parser = Parser::<patterns::module_decl::ModuleDeclPattern>::new(code.as_str());
    let module = package_parser.find_first_match().unwrap_or_else(|| panic!("error finding module declaration"));
    let module_name = module.name;

    // find the first safe-point to patch; usually right after the module declaration
    let safe_range = package_parser.find_next_line()
        .expect("file does not contain any module declaration");

    info!("patching file: {}", &manifest.file);
    info!("package found: {}", &module_name);

    // for each patch, find the target and patch it
    let (next_code, next_patches, next_imports) = manifest.patch.iter().fold(
        (code, patches, imports),
        |(code, mut patches, mut imports), manifest_patch| {
            let run_result = match manifest_patch.pattern.as_str() {
                "function_declaration" => {
                    run(
                        &mut Parser::<patterns::func_decl::FunctionDeclPattern>::new(code.as_str()),
                        &mut Parser::<patterns::func_decl::FunctionDeclPattern>::new(manifest_patch.code.as_str()),
                    )
                }
                "method_declaration" => {
                    run(
                        &mut Parser::<patterns::method_decl::MethodDeclPattern>::new(code.as_str()),
                        &mut Parser::<patterns::method_decl::MethodDeclPattern>::new(manifest_patch.code.as_str()),
                    )
                }

                // invalid pattern
                invalid_pattern => {
                    panic!("unknown pattern: {}", invalid_pattern);
                }
            };

            match run_result {
                // if match is found, return next_code and list of patches (to be appended at the end of the file)
                Some(next_code) => {
                    // collect imports to single import
                    let import_string = manifest_patch.imports.iter()
                        .map(|imp| {
                            vec![imp.alias.to_owned(), format!("\"{}\"", imp.path)].join(" ")
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

                    imports.push(import_string);
                    patches.push(manifest_patch.code.to_owned());

                    (next_code, patches, imports)
                }

                // if no match is found, return the original code and patches
                None => {
                    warn!("no matching pattern found for patch: {}", manifest_patch.code);
                    (code, patches, imports)
                }
            }
        }
    );

    // return
    Some(Result{
        module_name,
        safe_range,
        code: next_code,
        imports: next_imports,
        patches: next_patches,
    })
}

fn run<P: Pattern>(
    source_parser: &mut Parser<P>,
    target_parser: &mut Parser<P>,
) -> Option<String> {
    source_parser.find_and_patch(|pat| {
        pat.is_match(&target_parser
            .find_first_match()
            .unwrap_or_else(|| panic!("error finding target pattern")))
    })
}