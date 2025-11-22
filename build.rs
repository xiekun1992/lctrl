fn main() {
    println!("cargo:rerun-if-changed=src/native");
    // let lib_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    // println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path);
    // println!("cargo:rustc-link-search=.");

    #[cfg(target_os = "linux")]
    {
        let c_files = vec![
            "src/native/linux/input/input.c",
            "src/native/linux/input/listener.c",
            "src/native/linux/input/key.c",
            "src/native/linux/wayland/utils.c",
        ];
        cc::Build::new().files(c_files).compile("libcapture");
        println!("cargo:rustc-link-lib=dylib=wayland-client");
        println!("cargo:rustc-link-lib=dylib=evdev");
    }

    #[cfg(target_os = "windows")]
    {
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
    }

    #[cfg(target_os = "macos")]
    {
        let c_files = vec![
            "src/native/mac/utils.c",
            "src/native/mac/input/keyboard.c",
            "src/native/mac/input/listener.c",
            "src/native/mac/input/mouse.c",
        ];
        // println!("cargo:rerun-if-changed=src/native/mac/*.c");
        cc::Build::new()
            .files(c_files)
            .include("/System/Library/Frameworks/ApplicationServices.framework/Headers")
            .compile("libcapture");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
}
