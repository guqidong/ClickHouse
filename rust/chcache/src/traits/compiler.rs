pub trait CompilerMeta {
    const NAME: &'static str;
    fn from_args(args: Vec<String>) -> Box<dyn Compiler>;
}

pub trait Compiler {


    fn cacheable(&self) -> bool;
    fn cache_key(&self) -> String;

    fn compile(&self) -> Result<Vec<u8>, String>;
}
