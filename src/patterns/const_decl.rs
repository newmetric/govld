use crate::patterns::Pattern;
use tree_sitter::QueryMatch;

pub struct ConstDeclPattern {
    pub const_name: String,
    pub const_type: String,
    pub const_itself: String,
}

impl Pattern for ConstDeclPattern {
    fn ident(&self) -> String {
        self.const_name.to_owned()
    }

    fn sexp() -> &'static str {
        r#"
(source_file
	(const_declaration
    	(const_spec
        	name: (identifier) @name
            type: (type_identifier)? @type
        )
    ) @const_decl
)"#
    }

    fn from_match(matched: &QueryMatch, code: &str) -> Self {
        let const_itself = &code[matched.captures[0].node.byte_range()];
        let const_name = &code[matched.captures[1].node.byte_range()];
        let const_type = match matched.captures.get(2) {
            Some(cap) => &code[cap.node.byte_range()],
            None => "",
        };

        Self {
            const_itself: const_itself.to_owned(),
            const_name: const_name.to_owned(),
            const_type: const_type.to_owned(),
        }
    }

    fn append_suffix(matched: &QueryMatch, codebuf: &str) -> String {
        let struct_name_capture = matched.captures[1];
        let struct_name = &codebuf[struct_name_capture.node.byte_range()];

        let mut next = codebuf.to_owned();
        next.replace_range(
            struct_name_capture.node.byte_range(),
            &format!("{}_{}", struct_name, "_replaced_by_const_decl"),
        );
        next
    }

    fn delete(matched: &QueryMatch, codebuf: &str) -> String {
        let const_capture = matched.captures[0];

        let mut next = codebuf.to_owned();
        next.replace_range(const_capture.node.byte_range(), "");
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.const_name == other.const_name
    }
}
