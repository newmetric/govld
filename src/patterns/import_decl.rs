use crate::patterns::{Pattern};

const S_EXP: &str = r#"
(source_file
    (import_declaration
        (import_clause
            (identifier) @import_name
            (string) @import_path
        )
    )
)"#;
