use log::trace;

use crate::traits::compiler::{Compiler, CompilerMeta};

pub struct RustC {
    args: Vec<String>,
}

impl CompilerMeta for RustC {
    const NAME: &'static str = "rustc";

    fn from_args(args: Vec<String>) -> Box<dyn Compiler> {
        Box::new(RustC { args })
    }
}

// [thevar1able@homebox memchr-2.7.4]$ /home/thevar1able/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name memchr --edition=2021 /home/thevar1able/.cargo/registry/src/-6df83624996e3d27/memchr-2.7.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=117 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg feature=\"alloc\" --cfg feature=\"std\" --check-cfg "cfg(docsrs,test)" --check-cfg "cfg(feature, values(\"alloc\", \"compiler_builtins\", \"core\", \"default\", \"libc\", \"logging\", \"rustc-dep-of-std\", \"std\", \"use_std\"))" -C metadata=f0ff90587188d79c -C extra-filename=-5282d705ff339125 --out-dir /home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/x86_64-unknown-linux-gnu/debug/deps --target x86_64-unknown-linux-gnu -C linker=/usr/bin/clang -L dependency=/home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/x86_64-unknown-linux-gnu/debug/deps -L dependency=/home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/debug/deps --cap-lints allow -C link-arg=-fuse-ld=lld | head -n1
// {"$message_type":"artifact","artifact":"/home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/x86_64-unknown-linux-gnu/debug/deps/memchr-5282d705ff339125.d","emit":"dep-info"}
// {"$message_type":"artifact","artifact":"/home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/x86_64-unknown-linux-gnu/debug/deps/libmemchr-5282d705ff339125.rmeta","emit":"metadata"}
// {"$message_type":"artifact","artifact":"/home/thevar1able/nvmemount/clickhouse/cmake-build-debug/./cargo/build/x86_64-unknown-linux-gnu/debug/deps/libmemchr-5282d705ff339125.rlib","emit":"link"}
impl Compiler for RustC {
    fn cache_key(&self) -> String {
        self.args.join(" ")
    }

    fn cacheable(&self) -> bool {
        if self
            .args
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
        let out_dir = self
            .args
            .iter()
            .position(|x| x == "--out-dir")
            .map(|x| self.args[x + 1].clone())
            .unwrap();

        trace!("Out dir: {:?}", out_dir);

        let mut hasher = Hasher::new();
        rest_of_args.iter().map(|x| x.as_bytes()).for_each(|x| {
            hasher.update(&x);
        });
        let args_hash = hasher.finalize().to_string();

        let compiled_bytes: Vec<u8> = match get_from_fscache(&args_hash) {
            Some(bytes) => {
                info!("Local cache hit");

                let cursor = Cursor::new(bytes.clone());
                let mut archive = tar::Archive::new(cursor);
                archive.unpack(out_dir).expect("Unable to unpack tar");

                bytes
            }
            None => {
                trace!("Cache miss");

                let output = std::process::Command::new(compiler)
                    .args(&rest_of_args)
                    .output()
                    .unwrap();

                if !output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    std::process::exit(output.status.code().unwrap_or(1));
                }

                let files_to_pack = String::from_utf8_lossy(&output.stderr);
                eprintln!("{}", String::from_utf8_lossy(&output.stdout));

                let files_to_pack = files_to_pack
                    .lines()
                    .filter(|line| line.starts_with("{\"$message_type\":\"artifact\""))
                    .collect::<Vec<&str>>();

                let files_to_pack = files_to_pack
                    .iter()
                    .map(|x| {
                        let json: serde_json::Value = serde_json::from_str(x).unwrap();
                        let artifact = json["artifact"].as_str().unwrap();
                        let artifact = artifact.replace("\"", "");
                        artifact
                    })
                    .collect::<Vec<String>>();

                trace!("Files to pack: {:?}", files_to_pack);
                for (key, value) in std::env::vars() {
                    // trace!("Env var: {}: {}", key, value);
                    if key.starts_with("CARGO_") || key == "RUSTFLAGS" || key == "TARGET" {
                        trace!("Maybe interesting env var {}: {}", key, value);
                    }
                }

                let mut buffer = Vec::new();
                let cursor = Cursor::new(&mut buffer);
                let mut archive = tar::Builder::new(cursor);
                for file in files_to_pack {
                    let file = Path::new(&file);
                    let filename = file.strip_prefix(&out_dir).unwrap();
                    let filename = filename.to_str().unwrap();
                    trace!("Packing file: {}", file.display());
                    let mut packed_file = fs::File::open(file).unwrap();
                    archive.append_file(filename, &mut packed_file).unwrap();
                }
                archive.finish().unwrap();
                drop(archive);

                buffer
            }
        };

        write_to_fscache(&args_hash, &compiled_bytes);

        return;
    }

    fn passthrough() {
        let output = std::process::Command::new(compiler)
            .args(&rest_of_args)
            .output()
            .unwrap();

        if !output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(output.status.code().unwrap_or(1));
        }

        let output = String::from_utf8_lossy(&output.stdout);
        let output = output
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");
        println!("{}", output);
    }
}
