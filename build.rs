use std::env;
use std::path::Path;
use std::process::Command;

fn assemble(path: &str, out_dir: &str) {
    let src_file = Path::new(path).to_str().unwrap();
    let file_name = path.split('/').last().unwrap().replace(".asm", "");

    let s = Command::new("dependencies/vasm6502_oldstyle")
        .arg(format!("{}", src_file))
        .args(&["-dotdir", "-Fbin", "-o"])
        .arg(format!("{}/{}.bin", out_dir, file_name))
        .status()
        .expect("Failed to execute 6502 compiler.");

    if !s.success() {
        panic!("Compilation of 6502 assembly failed for file {}.", src_file)
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let files = ["src/demo/demo.asm"];

    for f in files.iter() {
        assemble(f, &out_dir);
        println!("cargo:rerun-if-changed={}", f)
    }
}
