use crate::patterns::Pattern;
use tree_sitter::QueryMatch;

pub struct VariableDeclPattern {
    pub var_name: String,
    pub var_itself: String,
}

impl Pattern for VariableDeclPattern {
    fn ident(&self) -> String {
        self.var_name.to_owned()
    }

    fn sexp() -> &'static str {
        r#"
(source_file
	(var_declaration
    	(var_spec
        	name: (identifier) @name
        )
    ) @var_decl
)"#
    }

    fn from_match(matched: &QueryMatch, code: &str) -> Self {
        let var_itself = &code[matched.captures[0].node.byte_range()];
        let var_name = &code[matched.captures[1].node.byte_range()];

        Self {
            var_itself: var_itself.to_owned(),
            var_name: var_name.to_owned(),
        }
    }

    fn append_suffix(matched: &QueryMatch, codebuf: &str) -> String {
        let struct_name_capture = matched.captures[1];
        let struct_name = &codebuf[struct_name_capture.node.byte_range()];

        let mut next = codebuf.to_owned();
        next.replace_range(
            struct_name_capture.node.byte_range(),
            &format!("{}_{}", struct_name, "_replaced_by_var_decl"),
        );
        next
    }

    fn delete(matched: &QueryMatch, codebuf: &str) -> String {
        let var_capture = matched.captures[0];

        let mut next = codebuf.to_owned();
        next.replace_range(var_capture.node.byte_range(), "");
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.var_name == other.var_name
    }
}
