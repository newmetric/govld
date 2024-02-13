use crate::patterns::Pattern;

const S_EXP: &str = r#"
(source_file
    (package_clause
      (package_identifier) @package)
)+"#;

pub struct ModuleDeclPattern {
    pub name: String,
}

impl Pattern for ModuleDeclPattern {
    fn ident(&self) -> String {
        self.name.clone()
    }

    fn sexp() -> &'static str {
        S_EXP
    }

    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self {
        let package_decl = &code[matched.captures[0].node.byte_range()];

        Self {
            name: package_decl.to_string(),
        }
    }

    fn append_suffix(_: &tree_sitter::QueryMatch, _: &str) -> String {
        panic!("ModuleDeclPattern::replace() not implemented")
    }

    fn delete(_: &tree_sitter::QueryMatch, _: &str) -> String {
        panic!("ModuleDeclPattern::delete() not implemented")
    }

    fn is_match(&self, _: &Self) -> bool {
        panic!("ModuleDeclPattern::is_match() not implemented")
    }
}
