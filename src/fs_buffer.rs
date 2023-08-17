use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FsBuffer {
    path_prefix: String,
    inner: HashMap<String, String>,
}

impl FsBuffer {
    pub fn new(path_prefix: String) -> Self {
        Self {
            path_prefix,
            inner: HashMap::new(),
        }
    }

    pub fn load(&mut self, file: String) -> String {
        match self.inner.get(&file) {
            Some(v) => v.to_owned(),
            None => self.load_from_file(file.clone()),
        }
    }

    // try loading from file; try joining from path prefix
    pub fn load_from_file(&mut self, path: String) -> String {
        let prefix_path = self.join_path(&self.path_prefix, &path);
        let content = std::fs::read_to_string(&prefix_path)
            .unwrap_or_else(|_| panic!("error opening file: {}", &prefix_path));
        self.inner.insert(path, content.clone());
        content
    }

    pub fn update(&mut self, path: &str, c: &str) {
        self.inner.insert(path.to_owned(), c.to_owned());
    }

    pub fn append_patch(&mut self, path: &str, patch: &str) {
        match self.inner.get_mut(path) {
            Some(v) => {
                append(v, patch);
            }
            None => {
                panic!("error patching file that was never loaded: {}", &path)
            }
        }
    }

    pub fn apply_patch_at(&mut self, path: &str, patch: &str, safe_range: &std::ops::Range<usize>) {
        match self.inner.get_mut(path) {
            Some(v) => {
                prepend(safe_range, v, patch);
            }
            None => {
                panic!("error patching file that was never loaded: {}", &path)
            }
        }
    }

    fn join_path(&self, prefix: &str, file: &str) -> String {
        PathBuf::from(prefix)
            .join(file)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn flush(&self) {
        for (path, content) in &self.inner {
            let fspath = self.join_path(&self.path_prefix, path);
            std::fs::write(fspath, content)
                .unwrap_or_else(|_| panic!("error writing file: {}", &path));
        }
    }
}

fn append(code: &mut String, patch: &str) {
    *code += "\n";
    *code += "// Patched by govld. DO NOT EDIT\n";
    *code += patch;
}

fn prepend(safe_range: &std::ops::Range<usize>, code: &mut String, patch: &str) {
    let end = safe_range.end;
    let patching = format!(
        r#"

// Patched by govld. DO NOT EDIT
{patch}

"#
    );

    code.insert_str(end, patching.as_str());
}
