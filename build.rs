use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=src/native");
    // let lib_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    // println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path);
    // println!("cargo:rustc-link-search=.");
    // let c_files_dir = Path::new("src/native/windows");

    #[cfg(target_os = "linux")]
    let c_files = vec![
        "src/native/linux/input/input.c",
        "src/native/linux/wayland/utils.c",
    ];
    // let c_files_dir = Path::new("src/native/linux");

    // let c_files = fs::read_dir(c_files_dir)
    //     .unwrap()
    //     .filter_map(Result::ok)
    //     .filter(|entry| {
    //         entry
    //             .path()
    //             .extension()
    //             .map(|ext| ext == "c")
    //             .unwrap_or(false)
    //     })
    //     .map(|entry| entry.path())
    //     .collect::<Vec<_>>();

    // println!("{:?}", c_files);
    #[cfg(target_os = "windows")]
    let c_files = vec![
        "src/native/windows/utils.c",
        "src/native/windows/clipboard/clipboard.c",
        "src/native/windows/input/helper.c",
        "src/native/windows/input/keyboard.c",
        "src/native/windows/input/listener.c",
        "src/native/windows/input/mouse.c",
        "src/native/windows/service/service.c",
    ];
    cc::Build::new().files(c_files).compile("libcapture");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=wayland-client");
}
