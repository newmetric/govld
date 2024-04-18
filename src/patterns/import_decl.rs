use crate::patterns::Pattern;

const S_EXP: &str = r#"
(source_file
    (import_declaration
        (import_spec_list
            (import_spec
                (package_identifier)? @import_name
                (interpreted_string_literal) @import_path
            ) @import_spec
        )
    )
)+"#;

const REPLACE_SUFFIX: &str = "_replaced_by_import_decl";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ImportDeclPattern {
    pub import_name: String,
    pub import_path: String,
    pub import_spec: String,
}

impl Pattern for ImportDeclPattern {
    fn ident(&self) -> String {
        self.import_path.clone()
    }

    fn sexp() -> &'static str {
        S_EXP
    }

    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self {
        let import_spec;
        let import_name;
        let import_path;

        let kinds = matched
            .captures
            .iter()
            .map(|c| c.node.kind())
            .collect::<Vec<_>>();

        match kinds.as_slice() {
            ["import_spec", "pacakge_identifier", "interpreted_string_literal"] => {
                import_spec = &code[matched.captures[0].node.byte_range()];
                import_name = &code[matched.captures[1].node.byte_range()];
                import_path = &code[matched.captures[2].node.byte_range()];
            }
            ["import_spec", "interpreted_string_literal"] => {
                import_spec = &code[matched.captures[0].node.byte_range()];
                import_name = "";
                import_path = &code[matched.captures[1].node.byte_range()];
            }

            other => unreachable!("invalid import_decl pattern: {:?}", other),
        }

        Self {
            import_name: import_name.to_string(),
            import_path: import_path.to_string(),
            import_spec: import_spec.to_string(),
        }
    }

    fn append_suffix(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String {
        let import_name_capture = matched.captures[1];
        let import_name = &codebuf[import_name_capture.node.byte_range()];

        let mut next = codebuf.to_string();
        next.replace_range(
            import_name_capture.node.byte_range(),
            &format!("{}_{}", import_name, REPLACE_SUFFIX),
        );
        next
    }

    fn delete(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String {
        let import_capture = matched.captures[0];

        let mut next = codebuf.to_string();
        next.replace_range(import_capture.node.byte_range(), "");
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.import_path == other.import_path
    }
}
