use crate::patterns::Pattern;
use tree_sitter::QueryMatch;

pub struct StructDeclPattern {
    pub name: String,
    pub fields: Vec<String>,
    pub struct_itself: String,
}

impl Pattern for StructDeclPattern {
    fn ident(&self) -> String {
        self.name.to_owned()
    }

    fn sexp() -> &'static str {
        r#"
(source_file
	(type_declaration
    	(type_spec
        	name: (type_identifier) @name
            type: (struct_type
            	(field_declaration_list
                	(field_declaration
                    	name: [(field_identifier)] @field_name
                    )
                )*
            )
        )
    ) @struct_decl
)"#
    }

    fn from_match(matched: &QueryMatch, code: &str) -> Self {
        let struct_itself = &code[matched.captures[0].node.byte_range()];
        let struct_name = &code[matched.captures[1].node.byte_range()];
        let fields = match matched.captures.get(2) {
            Some(cap) => &code[cap.node.byte_range()],
            None => "",
        };

        Self {
            struct_itself: struct_itself.to_owned(),
            name: struct_name.to_owned(),
            fields: vec![fields.to_owned()],
        }
    }

    fn replace(matched: &QueryMatch, codebuf: &str) -> String {
        let struct_name_capture = matched.captures[1];
        let struct_name = &codebuf[struct_name_capture.node.byte_range()];

        let mut next = codebuf.to_owned();
        next.replace_range(
            struct_name_capture.node.byte_range(),
            &format!("{}_{}", struct_name, "_replaced_by_struct_decl"),
        );
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
