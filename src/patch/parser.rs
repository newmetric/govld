use crate::patterns::Pattern;
use std::marker::PhantomData;
use std::ops::Range;

pub struct Parser<P> {
    code: String,
    tree: tree_sitter::Tree,
    language: tree_sitter::Language,

    _pattern: PhantomData<P>,
}

impl<P: Pattern> Parser<P> {
    pub fn new(code: &str) -> Self {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_go::language();

        parser
            .set_language(&language)
            .expect("error loading Go grammar");

        let tree = parser.parse(code, None).unwrap();

        Self {
            code: code.to_owned(),
            language,
            tree,
            _pattern: PhantomData,
        }
    }

    // find_first_match finds the first s_exp match in the code
    // useful for searching for a single match for a target pattern
    pub fn find_first_match(&self) -> Option<P> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(&self.language, P::sexp()).expect("query is invalid");

        cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];
                std::iter::once(slice)
            })
            .map(|m| P::from_match(&m, &self.code))
            .next()
    }

    pub fn find_next_line(&self) -> Option<Range<usize>> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(&self.language, P::sexp()).expect("query is invalid");

        let last = cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];
                std::iter::once(slice)
            })
            .last()?;

        Some(last.captures.last()?.node.byte_range())
    }

    // find_and_patch replaces original codebuffer if matching pattern is found in the source
    // otherwise append at the bottom
    pub fn find_and_patch(&self, predicate: impl Fn(&P) -> bool) -> Option<String> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(&self.language, P::sexp()).expect("query is invalid");

        cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];

                std::iter::once(slice)
            })
            .find_map(|m| {
                let patt = P::from_match(&m, &self.code);

                predicate(&patt).then(|| P::append_suffix(&m, &self.code))
            })
    }

    pub fn find_and_delete(&self, predicate: impl Fn(&P) -> bool) -> Option<String> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(&self.language, P::sexp()).expect("query is invalid");

        cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];

                std::iter::once(slice)
            })
            .find_map(|m| {
                let patt = P::from_match(&m, &self.code);

                predicate(&patt).then(|| P::delete(&m, &self.code))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::patterns::{self, func_decl::FunctionDeclPattern};

    #[test]
    fn test_parser() {
        let source = include_str!("./test_parser.go");
        let parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(source);
        let next = parser.find_first_match();

        assert_eq!(
            next,
            Some(FunctionDeclPattern {
                name: "internal".to_owned(),
                param_t: "()".to_owned(),
                return_t: String::new(),
                fn_itself: "func internal() {\n\tprintln(\"Hello, Foo!\")\n}".to_owned(),
            })
        );
    }

    #[test]
    fn test_find_and_replace() {
        let source = include_str!("./test_parser.go");

        let patch: &str = r#"
func internal() {
println("Hello, World!")
}
        "#;

        let patch_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(patch);
        let patch_target = patch_parser.find_first_match().unwrap();

        assert_eq!(
            patch_target,
            FunctionDeclPattern {
                name: "internal".to_owned(),
                param_t: "()".to_owned(),
                return_t: String::new(),
                fn_itself: "func internal() {\nprintln(\"Hello, World!\")\n}".to_owned(),
            }
        );

        let source_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(source);
        let result = source_parser.find_and_patch(|f| f.name == patch_target.name);

        let expected = "package main\n\nfunc internal__replaced_by_function_decl() {\n\tprintln(\"Hello, Foo!\")\n}\nfunc internal2() {\n\tprintln(\"Hello, Foo!\")\n}\n";
        assert_eq!(result, Some(expected.to_owned()))
    }

    #[test]
    fn test_find_and_delete() {
        let source = include_str!("./test_parser.go");

        let patch: &str = r#"func internal() {}"#;

        let patch_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(patch);
        let patch_target = patch_parser.find_first_match().unwrap();

        assert_eq!(
            patch_target,
            FunctionDeclPattern {
                name: "internal".to_owned(),
                param_t: "()".to_owned(),
                return_t: String::new(),
                fn_itself: "func internal() {}".to_owned(),
            }
        );

        let source_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(source);
        let result = source_parser.find_and_delete(|f| f.name == patch_target.name);

        let expected = "package main\n\n\nfunc internal2() {\n\tprintln(\"Hello, Foo!\")\n}\n";
        assert_eq!(result, Some(expected.to_owned()))
    }
}
