use std::env;

fn main() {
    let lib_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path);
    println!("cargo:rustc-link-search=.");
}
