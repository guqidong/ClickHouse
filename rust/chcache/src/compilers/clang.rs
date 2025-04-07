use crate::traits::compiler::Compiler;
use crate::traits::compiler::CompilerMeta;

pub struct Clang {
    args: Vec<String>,
}

impl CompilerMeta for Clang {
    const NAME: &'static str = "clang";
    fn from_args(args: Vec<String>) -> Box<dyn Compiler> {
        Box::new(Clang { args })
    }
}

impl Compiler for Clang {
    fn compile(&self) -> Result<Vec<u8>, String> {
        let output = std::process::Command::new("clang")
            .args(&self.args)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        Ok(vec![])
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

// impl Compiler for ClangXX {
// }
