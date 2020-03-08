use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded.rs");

    let code = format!("pub fn get() -> &'static str {{
        \"{}\"
    }}
    ", target);

    fs::write(&dest_path, &code).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
