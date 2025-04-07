use crate::compiler::Compiler;

pub struct Rustc {
    args: Vec<String>,
}

impl Compiler for Rustc {
    fn from_args(args: Vec<String>) -> Self {
        Rustc {
            args,
        }
    }

    fn name(&self) -> &str {
        "rustc"
    }

    fn cache_key(&self) -> String {
        self.args.join(" ")
    }

    fn cacheable(&self) -> bool {
        if self.args
            .iter()
            .any(|arg| arg == "--version" || arg == "--help" || arg == "--explain" || arg == "-vV")
        {
            return false;
        }

        if self.args.iter().any(|arg| arg == "--print") {
            return false;
        }

        let has_input = self.args.iter().any(|arg| arg.ends_with(".rs"));
        if !has_input {
            return false;
        }

        // if self.args.iter().any(|arg| arg.contains("emit=link")) {
        //     return false;
        // }

        true
    }

    fn compile(&self, source: &str) -> Result<Vec<u8>, String> {
        // Simulate compilation process
        if source.is_empty() {
            return Err("Source code is empty".to_string());
        }
        println!("Compiling {}", source);

        return Ok(vec![
            0x7F, 0x45, 0x4C, 0x46, // ELF magic number
            // ... (rest of the ELF header)
        ]);
    }
}
