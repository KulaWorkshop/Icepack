#[cfg(windows)]
fn main() {
    let dst = cmake::build("lib/lzrw");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lzrw");

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}

#[cfg(unix)]
fn main() {
    let dst = cmake::build("lib/lzrw");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lzrw");
}
