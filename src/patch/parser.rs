use std::marker::PhantomData;
use crate::patterns;

pub struct Parser<P: patterns::Pattern> {
    pub code: String,
    tree: tree_sitter::Tree,
    language: tree_sitter::Language,
    _p: PhantomData<P>,
}

impl <P: patterns::Pattern> Parser<P> {
    pub fn new(code: &str) -> Self {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_go::language();

        parser.set_language(language).expect("error loading Go grammar");

        let tree = parser.parse(code, None).unwrap();

        Self {
            code: String::from(code),
            language,
            tree,
            _p: Default::default(),
        }
    }

    // find_first_match finds the first s_exp match in the code
    // useful for searching for a single match for a target pattern
    pub fn find_first_match(&self) -> Option<P> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(
            self.language,
            P::sexp(),
        ).expect("query is invalid");

        cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];
                std::iter::once(slice)
            })
            .find_map(|m| Some(P::from_match(&m, &self.code)))
    }

    pub fn find_and_patch<Predicate: Fn(&P) -> bool>(&mut self, predicate: Predicate) -> Option<String> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let query = tree_sitter::Query::new(
            self.language,
            P::sexp(),
        ).expect("query is invalid");

        let next = cursor
            .matches(&query, self.tree.root_node(), |node: tree_sitter::Node| {
                let cb = self.code.as_bytes();
                let slice = &cb[node.byte_range()];
                std::iter::once(slice)
            })
            .find_map(|m| {
                let patt = P::from_match(&m, &self.code);

                predicate(&patt).then(|| P::replace(&m, &self.code))
            });

        next
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::patterns;

    #[test]
    fn test_parser() {
        let source: &str = r#"package main

func internal() {
println("Hello, Foo!")
}
func internal2() {
println("Hello, Foo!")
}
        "#;
        let parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(source);
        let next = parser.find_first_match();

        assert!(next.is_some());
    }

    #[test]
    fn test_find_and_replace() {
        let source: &str = r#"package main

func internal() {
println("Hello, Foo!")
}
func internal2() {
println("Hello, Foo!")
}
    "#;

        let patch: &str = r#"
func internal() {
println("Hello, World!")
}
        "#;

        let patch_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(patch);
        let patch_target = patch_parser.find_first_match().unwrap();

        dbg!(&patch_target);

        let mut source_parser = Parser::<patterns::func_decl::FunctionDeclPattern>::new(source);
        source_parser.find_and_patch(|f| {
            f.name == patch_target.name
        });

        print!("{}", source_parser.finalize());
    }
}
