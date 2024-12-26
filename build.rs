use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get the output directory for the build script
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir);

    // Define the source directory
    let src_dir = Path::new("src");

    // Recursively find all `.txt` files in `src/dayN`
    let txt_files = find_txt_files(src_dir, src_dir).expect("Failed to find .txt files");

    for (src, rel_path) in txt_files {
        let dest = out_path.join(rel_path);

        // Create parent directories in the destination path
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).expect("Failed to create destination directory");
        }

        // Copy the file
        fs::copy(&src, &dest).expect("Failed to copy file");
    }
}

/// Recursively finds all `.txt` files in the given directory, returning a vector
/// of tuples (absolute source path, relative path from the base directory).
fn find_txt_files(base_dir: &Path, dir: &Path) -> std::io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut txt_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into directories
            txt_files.extend(find_txt_files(&base_dir, &path)?);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            // Collect `.txt` files
            let rel_path = path.strip_prefix(base_dir).unwrap().to_path_buf();
            txt_files.push((path, rel_path));
        }
    }

    Ok(txt_files)
}
