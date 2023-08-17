use crate::patterns::Pattern;
use tree_sitter::QueryMatch;

pub struct InterfaceDeclPattern {
    pub name: String,
    pub fields: Vec<String>,
    pub interface_itself: String,
}

impl Pattern for InterfaceDeclPattern {
    fn ident(&self) -> String {
        self.name.to_owned()
    }

    fn sexp() -> &'static str {
        r#"
(source_file
	(type_declaration
    	(type_spec
        	name: (type_identifier) @name
            type: (interface_type
            	(method_spec
                	name: [(field_identifier)] @field_name
                )*
            )
        )
    ) @interface_decl
)"#
    }

    fn from_match(matched: &QueryMatch, code: &str) -> Self {
        let interface_itself = &code[matched.captures[0].node.byte_range()];
        let struct_name = &code[matched.captures[1].node.byte_range()];
        let fields = match matched.captures.get(2) {
            Some(cap) => &code[cap.node.byte_range()],
            None => "",
        };

        Self {
            interface_itself: interface_itself.to_owned(),
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
            &format!("{}_{}", struct_name, "_replaced_by_interface_decl"),
        );
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::InterfaceDeclPattern;
    use crate::patch::parser::Parser;

    #[test]
    fn test_struct_decl_pattern() {
        let code = r#"
package internal

type Foo struct {
	kkk int
	aaa a.Pointer
}

type Xyz struct {
	wtf int
}
        "#;

        let p = Parser::<InterfaceDeclPattern>::new(code);
        let fm = p.find_first_match().unwrap();

        dbg!(fm.name, fm.fields);
    }
}
