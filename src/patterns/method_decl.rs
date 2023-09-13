use crate::patterns::{method_decl_with_receiver_name, method_decl_without_receiver_name, Pattern};

pub const S_EXP: &str = r#"
(source_file
    (method_declaration
    	receiver: (parameter_list
        	(parameter_declaration
            	name: (identifier)? @receiver_name
                type: (type_identifier) @receiver_type
            )?
            (parameter_declaration
            	name: (identifier)? @receiver_name
                type: (pointer_type
                	(type_identifier) @receiver_type
                )
            )?
        )
        name: (field_identifier) @name
        parameters: (parameter_list) @params
        result: (type_identifier)? @return
    )
)+"#;

pub const REPLACE_SUFFIX: &str = "_replaced_by_method_decl";

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MethodDeclPattern {
    WithReceiverName(method_decl_with_receiver_name::MethodDeclPatternWithReceiverName),
    WithoutReceiverName(method_decl_without_receiver_name::MethodDeclPatternWithoutReceiverName),
}

impl Pattern for MethodDeclPattern {
    fn ident(&self) -> String {
        match self {
            MethodDeclPattern::WithReceiverName(k) => k.name.clone(),
            MethodDeclPattern::WithoutReceiverName(k) => k.name.clone(),
        }
    }

    fn sexp() -> &'static str {
        S_EXP
    }

    fn from_match(matched: &tree_sitter::QueryMatch, code: &str) -> Self {
        match is_receiver_name_present(matched) {
            true => MethodDeclPattern::WithoutReceiverName(
                method_decl_without_receiver_name::MethodDeclPatternWithoutReceiverName::from_match(
                    matched, code,
                ),
            ),
            false => MethodDeclPattern::WithReceiverName(
                method_decl_with_receiver_name::MethodDeclPatternWithReceiverName::from_match(
                    matched, code,
                ),
            ),
        }
    }

    fn replace(matched: &tree_sitter::QueryMatch, codebuf: &str) -> String {
        match is_receiver_name_present(matched) {
            true => {
                method_decl_without_receiver_name::MethodDeclPatternWithoutReceiverName::replace(
                    matched, codebuf,
                )
            }
            false => method_decl_with_receiver_name::MethodDeclPatternWithReceiverName::replace(
                matched, codebuf,
            ),
        }
    }

    fn is_match(&self, other: &Self) -> bool {
        match self {
            MethodDeclPattern::WithReceiverName(k) => match other {
                MethodDeclPattern::WithReceiverName(o) => k.is_match(o),
                _ => false,
            },
            MethodDeclPattern::WithoutReceiverName(k) => match other {
                MethodDeclPattern::WithoutReceiverName(o) => k.is_match(o),
                _ => false,
            },
        }
    }
}

fn is_receiver_name_present(matched: &tree_sitter::QueryMatch) -> bool {
    matched.captures.len() < 5
}
