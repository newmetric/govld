pub mod func_decl;
pub mod module_decl;
pub mod method_decl;
mod import_decl;

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