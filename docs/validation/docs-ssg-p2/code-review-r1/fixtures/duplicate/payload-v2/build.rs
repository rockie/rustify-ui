use std::{fs, env, path::PathBuf};
fn main() {
 println!("cargo:rerun-if-changed=payload.txt");
 let value = fs::read_to_string("payload.txt").unwrap();
 let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
 fs::write(out.join("value.rs"), format!("pub const VALUE: i32 = {};\n", value.trim())).unwrap();
}
