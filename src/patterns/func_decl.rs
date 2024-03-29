use crate::patterns::Pattern;

const S_EXP: &str = r#"
(source_file
    (function_declaration
        name: (identifier) @name
        parameters: (parameter_list) @params
        result: (type_identifier)? @return
    ) @func_decl
)+"#;

const REPLACE_SUFFIX: &str = "_replaced_by_function_decl";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionDeclPattern {
    pub name: String,
    pub param_t: String,
    pub return_t: String,
    pub fn_itself: String,
}

impl Pattern for FunctionDeclPattern {
    fn ident(&self) -> String {
        self.name.clone()
    }

    fn sexp() -> &'static str {
        S_EXP
    }

    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self {
        let fn_itself = &code[matched.captures[0].node.byte_range()];
        let fn_name = &code[matched.captures[1].node.byte_range()];
        let fn_param_t = &code[matched.captures[2].node.byte_range()];
        let fn_return_t = match matched.captures.get(3) {
            Some(cap) => &code[cap.node.byte_range()],
            None => "",
        };

        Self {
            name: fn_name.to_string(),
            param_t: fn_param_t.to_string(),
            return_t: fn_return_t.to_string(),
            fn_itself: fn_itself.to_string(),
        }
    }

    fn append_suffix(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String {
        let fn_name_capture = matched.captures[1];
        let fn_name = &codebuf[fn_name_capture.node.byte_range()];

        let mut next = codebuf.to_string();
        next.replace_range(
            fn_name_capture.node.byte_range(),
            &format!("{}_{}", fn_name, REPLACE_SUFFIX),
        );
        next
    }

    fn delete(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String {
        let fn_capture = matched.captures[0];

        let mut next = codebuf.to_string();
        next.replace_range(fn_capture.node.byte_range(), "");
        next
    }

    fn is_match(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
