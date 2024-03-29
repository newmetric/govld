use crate::patch::parser::Parser;

use crate::manifest::PatchType;

pub mod func_decl;
pub mod method_decl;
pub mod module_decl;

mod import_decl;
mod interface_decl;
mod struct_decl;
mod variable_decl;

pub trait Pattern
where
    Self: Sized,
{
    // ident returns whatever human-readable identifier for the pattern.
    fn ident(&self) -> String;

    // sexp returns S_EXP for the pattern.
    fn sexp() -> &'static str;

    // hehe
    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self;
    fn append_suffix(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String;
    fn delete(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String;
    fn is_match(&self, other: &Self) -> bool;
}

pub fn try_run(
    pattern: &str,
    code: String,
    patch: String,
    patch_type: Option<&PatchType>,
) -> Option<String> {
    macro_rules! run {
        ($ty: ty) => {
            run(
                &Parser::<$ty>::new(&code),
                &Parser::<$ty>::new(&patch),
                patch_type,
            )
        };
    }

    match pattern {
        "function_declaration" => run!(func_decl::FunctionDeclPattern),
        "method_declaration" => run!(method_decl::MethodDeclPattern),
        "struct_declaration" => run!(struct_decl::StructDeclPattern),
        "interface_declaration" => run!(interface_decl::InterfaceDeclPattern),
        "variable_declaration" => run!(variable_decl::VariableDeclPattern),
        _ => panic!("unknown pattern: {}", pattern),
    }
}

pub fn run<P: Pattern>(
    source_parser: &Parser<P>,
    target_parser: &Parser<P>,
    patch_type: Option<&PatchType>,
) -> Option<String> {
    match patch_type {
        Some(PatchType::Overwrite) => source_parser.find_and_delete(|pat| {
            pat.is_match(
                &target_parser
                    .find_first_match()
                    .unwrap_or_else(|| panic!("error finding target pattern")),
            )
        }),
        // default behavior
        _ => source_parser.find_and_patch(|pat| {
            pat.is_match(
                &target_parser
                    .find_first_match()
                    .unwrap_or_else(|| panic!("error finding target pattern")),
            )
        }),
    }
}
