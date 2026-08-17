use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Discovers all `.rl` files in a given directory, recursively, and sorts them alphabetically.
/// If the provided path is a file, it returns just that file.
pub fn find_rl_files(base_path: &Path) -> Vec<PathBuf> {
    if base_path.is_file() {
        return vec![base_path.to_path_buf()];
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rl") {
            files.push(path.to_path_buf());
        }
    }

    // Spec §5: execute them in alphabetical order
    files.sort();
    
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_find_rl_files() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        // Create some files
        File::create(path.join("b.rl")).unwrap();
        File::create(path.join("a.rl")).unwrap();
        File::create(path.join("c.txt")).unwrap(); // Should be ignored

        let sub_dir = path.join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        File::create(sub_dir.join("z.rl")).unwrap();

        let files = find_rl_files(path);

        assert_eq!(files.len(), 3);
        // Ensure alphabetical order
        assert!(files[0].ends_with("a.rl"));
        assert!(files[1].ends_with("b.rl"));
        assert!(files[2].ends_with("z.rl"));
    }
}
