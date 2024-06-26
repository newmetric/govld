use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct FsBuffer<'path> {
    path_prefix: &'path Path,
    inner: HashMap<String, String>,
}

impl<'path> FsBuffer<'path> {
    pub fn new(path_prefix: &'path Path) -> Self {
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

    pub fn try_load(&mut self, file: String) -> Option<String> {
        match self.inner.get(&file) {
            Some(v) => Some(v.to_owned()),
            None => self.try_load_from_file(file.clone()),
        }
    }

    // try loading from file; try joining from path prefix
    pub fn load_from_file(&mut self, path: String) -> String {
        let prefix_path = self.join_path(&path);
        let content = std::fs::read_to_string(&prefix_path)
            .unwrap_or_else(|_| panic!("error opening file: {}", &prefix_path));
        self.inner.insert(path, content.clone());
        content
    }

    pub fn try_load_from_file(&mut self, path: String) -> Option<String> {
        let prefix_path = self.join_path(&path);
        let content = std::fs::read_to_string(prefix_path).ok()?;
        self.inner.insert(path, content.clone());
        Some(content)
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

    fn join_path(&self, file: &str) -> String {
        self.path_prefix.join(file).to_str().unwrap().to_string()
    }

    pub fn flush(&self) {
        for (path, content) in &self.inner {
            let fspath = self.join_path(path);
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
