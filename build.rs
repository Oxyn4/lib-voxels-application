use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Locate top-level project directory
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Go up the directory tree to find the actual binary root (not your crate!)
    let project_root = find_project_root(&manifest_dir).unwrap_or(manifest_dir);

    let config_path = project_root.join("voxels.toml");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("voxels.toml");

    if config_path.exists() {
        let content = fs::read_to_string(&config_path).expect("Failed to read voxels.toml");
        fs::write(&out_path, content).expect("Failed to copy voxels.toml");
    } else {
        panic!(
            "voxels.toml not found in project root: {}",
            project_root.display()
        );
    }

    // Tell Rust to rerun build.rs if the config changes
    println!("cargo:rerun-if-changed={}", config_path.display());
}

// Heuristically find the outermost Cargo project root
fn find_project_root(start: &PathBuf) -> Option<PathBuf> {
    let mut dir = start.clone();
    while dir.parent().is_some() {
        if dir.join("Cargo.toml").exists() && dir.join("src").join("main.rs").exists() {
            return Some(dir);
        }
        dir = dir.parent().unwrap().to_path_buf();
    }
    None
}