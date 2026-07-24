fn main() {
    // The Windows executable embeds this resource at compile time. Track it so
    // `tauri dev` rebuilds the executable after an application-icon update.
    println!("cargo:rerun-if-changed=icons/icon.ico");
    tauri_build::build()
}
