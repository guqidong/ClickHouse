use crate::compiler::Compiler;

pub struct Clang {
    args: Vec<String>,
}

impl Compiler for Clang {
    fn from_args(args: Vec<String>) -> Self {
        Self { args }
    }

    fn name(&self) -> &str {
        "clang"
    }

    fn compile(&self) -> Result<Vec<u8>, String> {
        let output = std::process::Command::new("clang")
            .args(&self.args)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        Ok(())
    }

    fn cache_key(&self) -> String {
        let mut key = String::new();
        for arg in &self.args {
            key.push_str(arg);
            key.push('\n');
        }
        key
    }

    fn cacheable(&self) -> bool {
        true
    }
}
