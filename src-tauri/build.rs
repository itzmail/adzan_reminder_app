use std::env;
use std::fs;
use std::path::Path;

fn main() {
    #[cfg(feature = "gui")]
    {
        tauri_build::build();

        copy_assets();
    }

    #[cfg(not(feature = "gui"))]
    {
        println!("cargo:rerun-if-changed=build.rs");
    }
}

fn copy_assets() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let assets_src = Path::new(&manifest_dir).join("assets");
    let profile = env::var("PROFILE").unwrap_or("debug".to_string());
    let target_dir = Path::new(&manifest_dir)
        .join("target")
        .join(profile)
        .join("assets");

    if assets_src.exists() {
        // Hapus target assets kalau sudah ada
        let _ = fs::remove_dir_all(&target_dir);

        // Copy seluruh folder assets
        fs::create_dir_all(&target_dir).unwrap();
        for entry in fs::read_dir(&assets_src).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let dest_path = target_dir.join(&file_name);
            fs::copy(entry.path(), &dest_path).unwrap();
        }
        println!("cargo:rerun-if-changed=assets/");
        println!("cargo:rustc-env=ADZAN_ASSETS_DIR={}", target_dir.display());
    }
}
