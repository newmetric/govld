use crate::patch::parser::Parser;

pub mod func_decl;
pub mod module_decl;
pub mod method_decl;
mod import_decl;
mod struct_decl;

pub trait Pattern where Self: Sized {
    // ident returns whatever human-readable identifier for the pattern.
    fn ident(&self) -> String;

    // sexp returns S_EXP for the pattern.
    fn sexp() -> &'static str;

    // hehe
    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self;
    fn replace(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String;
    fn is_match(&self, other: &Self) -> bool;
}

pub fn try_run(pattern: &str, code: String, patch: String) -> Option<String> {
    match pattern {
        "function_declaration" => {
            run(
                &mut Parser::<func_decl::FunctionDeclPattern>::new(code.as_str()),
                &mut Parser::<func_decl::FunctionDeclPattern>::new(patch.as_str()),
            )
        }
        "method_declaration" => {
            run(
                &mut Parser::<method_decl::MethodDeclPattern>::new(code.as_str()),
                &mut Parser::<method_decl::MethodDeclPattern>::new(patch.as_str()),
            )
        }
        "struct_declaration" => {
            run(
                &mut Parser::<struct_decl::StructDeclPattern>::new(code.as_str()),
                &mut Parser::<struct_decl::StructDeclPattern>::new(patch.as_str()),
            )
        }
        _ => panic!("unknown pattern: {}", pattern),
    }
}

pub fn run<P: Pattern>(
    source_parser: &mut Parser<P>,
    target_parser: &mut Parser<P>,
) -> Option<String> {
    source_parser.find_and_patch(|pat| {
        pat.is_match(&target_parser
            .find_first_match()
            .unwrap_or_else(|| panic!("error finding target pattern")))
    })
}