use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FsBuffer {
    path_prefix: String,
    inner: HashMap<String, String>
}

impl FsBuffer {
    pub fn new(path_prefix: String) -> Self {
        Self {
            path_prefix,
            inner: HashMap::new()
        }
    }


    pub fn load(&mut self, file: String) -> String {
        match self.inner.get(&file) {
            Some(v) => v.clone(),
            None => {
                self.load_from_file(file.clone())
            }
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

    pub fn update(&mut self, path: String, c: String) {
        self.inner.insert(path, c);
    }

    pub fn update_patch(&mut self, module_name: String, path: String, c: String) {
        let content = match self.inner.get(&path) {
            Some(v) => v.clone(),
            None => init_patch_package(module_name)
        };
        let content = format!("{}\n{}", content, c);

        self.inner.insert(path, content);
    }

    fn join_path(&self, prefix: &String, file: &String) -> String {
        PathBuf::from(prefix).join(file).to_str().unwrap().to_string()
    }

    pub fn flush(&self) {
        for (path, content) in &self.inner {
            let fspath = self.join_path(&self.path_prefix, path);
            std::fs::write(fspath, content).unwrap_or_else(|_| panic!("error writing file: {}", &path));
        }
    }
}

fn init_patch_package(module_name: String) -> String {
    format!(r#"
package {module_name}

"#)
}