pub trait Compiler {
    fn from_args(args: Vec<String>) -> Self;

    fn name(&self) -> &str;
    fn cacheable(&self) -> bool;
    fn cache_key(&self) -> String;

    fn compile(&self) -> Result<Vec<u8>, String>;
}
