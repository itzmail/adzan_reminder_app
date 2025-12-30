fn main() {
    #[cfg(feature = "gui")]
    {
        tauri_build::build()
    }

    #[cfg(not(feature = "gui"))]
    {
        println!("cargo:rerun-if-changed=build.rs");
    }
}
