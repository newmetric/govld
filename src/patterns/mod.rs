pub mod func_decl;
pub mod module_decl;

#[derive(Debug, Eq, PartialEq)]
pub enum Patterns {
    FunctionDecl,
}

impl From<&str> for Patterns {
    fn from(s: &str) -> Self {
        match s {
            "function_declaration" => Patterns::FunctionDecl,
            _ => panic!("Unknown pattern: {}", s),
        }
    }
}

pub trait Pattern where Self: Sized {
    fn from_match(matched: &tree_sitter::QueryMatch, code: &String) -> Self;
    fn sexp() -> &'static str;
    fn replace(matched: &tree_sitter::QueryMatch, codebuf: &String) -> String;
    fn is_match(&self, other: &Self) -> bool;
}