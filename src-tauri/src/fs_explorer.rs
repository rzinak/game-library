use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    /// True for Unix executables (mode & 0o111) or Windows .exe files.
    pub is_executable: bool,
    /// True for macOS .app bundles (directory ending in ".app").
    pub is_app_bundle: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Bookmark {
    pub label: String,
    pub path: String,
}

/// Reads the entries of `path`, hides dot-files, sorts directories first
/// then alphabetically, and follows symlinks for metadata.
pub fn read_dir(path: &str) -> Result<Vec<DirEntry>, String> {
    let iter = std::fs::read_dir(path).map_err(|e| e.to_string())?;

    let mut entries: Vec<DirEntry> = iter
        .filter_map(|r| r.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                return None;
            }
            // Follow symlinks so .app bundles report is_dir = true
            let meta = std::fs::metadata(entry.path()).ok()?;
            let is_dir = meta.is_dir();
            let is_app_bundle = is_dir && name.ends_with(".app");
            let is_executable = !is_dir && check_executable(entry.path().as_path());
            Some(DirEntry {
                name,
                path: entry.path().to_string_lossy().to_string(),
                is_dir,
                is_executable,
                is_app_bundle,
            })
        })
        .collect();

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

/// Returns platform-appropriate starting locations for the file browser.
pub fn get_bookmarks() -> Vec<Bookmark> {
    let mut bm: Vec<Bookmark> = Vec::new();

    #[cfg(target_os = "macos")]
    {
        push_if_exists(&mut bm, "Applications", "/Applications");
        if let Ok(home) = std::env::var("HOME") {
            push_if_exists(&mut bm, "Home", &home);
            push_if_exists(&mut bm, "~/Applications", &format!("{home}/Applications"));
            push_if_exists(&mut bm, "Downloads", &format!("{home}/Downloads"));
            push_if_exists(&mut bm, "Desktop", &format!("{home}/Desktop"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            push_if_exists(&mut bm, "Home", &home);
            push_if_exists(&mut bm, "Games", &format!("{home}/Games"));
            push_if_exists(&mut bm, "Downloads", &format!("{home}/Downloads"));
        }
        for path in ["/usr/games", "/usr/local/games", "/opt"] {
            push_if_exists(&mut bm, path, path);
        }
    }

    #[cfg(target_os = "windows")]
    {
        push_if_exists(&mut bm, "Program Files", "C:\\Program Files");
        push_if_exists(&mut bm, "Program Files (x86)", "C:\\Program Files (x86)");
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            push_if_exists(&mut bm, "Local AppData", &appdata);
        }
        if let Ok(home) = std::env::var("USERPROFILE") {
            push_if_exists(&mut bm, "Home", &home);
            push_if_exists(&mut bm, "Desktop", &format!("{home}\\Desktop"));
            push_if_exists(&mut bm, "Downloads", &format!("{home}\\Downloads"));
        }
    }

    bm
}

fn push_if_exists(bookmarks: &mut Vec<Bookmark>, label: &str, path: &str) {
    if Path::new(path).exists() {
        bookmarks.push(Bookmark {
            label: label.to_string(),
            path: path.to_string(),
        });
    }
}

#[cfg(unix)]
fn check_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn check_executable(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("exe"))
        .unwrap_or(false)
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_dir() -> std::path::PathBuf {
        let p = std::env::temp_dir()
            .join(format!("fs_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn read_nonexistent_dir_returns_error() {
        assert!(read_dir("/no/such/path_xyzzy_test").is_err());
    }

    #[test]
    fn read_dir_sorts_dirs_before_files() {
        let dir = tmp_dir();
        fs::write(dir.join("z_file.txt"), "").unwrap();
        fs::create_dir(dir.join("a_folder")).unwrap();

        let entries = read_dir(dir.to_str().unwrap()).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries[0].is_dir, "first entry should be directory");
        assert_eq!(entries[0].name, "a_folder");
        assert_eq!(entries[1].name, "z_file.txt");

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn read_dir_hides_dotfiles() {
        let dir = tmp_dir();
        fs::write(dir.join(".hidden"), "").unwrap();
        fs::write(dir.join("visible"), "").unwrap();

        let entries = read_dir(dir.to_str().unwrap()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "visible");

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn read_dir_is_alphabetical_within_type() {
        let dir = tmp_dir();
        fs::create_dir(dir.join("c_dir")).unwrap();
        fs::create_dir(dir.join("a_dir")).unwrap();
        fs::write(dir.join("z.txt"), "").unwrap();
        fs::write(dir.join("b.txt"), "").unwrap();

        let entries = read_dir(dir.to_str().unwrap()).unwrap();
        assert_eq!(entries[0].name, "a_dir");
        assert_eq!(entries[1].name, "c_dir");
        assert_eq!(entries[2].name, "b.txt");
        assert_eq!(entries[3].name, "z.txt");

        fs::remove_dir_all(dir).ok();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn app_bundle_directory_marked_as_app_bundle() {
        let dir = tmp_dir();
        let bundle = dir.join("MyGame.app");
        fs::create_dir(&bundle).unwrap();
        fs::write(dir.join("readme.txt"), "").unwrap();

        let entries = read_dir(dir.to_str().unwrap()).unwrap();
        let app = entries.iter().find(|e| e.name == "MyGame.app").unwrap();
        assert!(app.is_dir);
        assert!(app.is_app_bundle);

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn bookmarks_are_non_empty() {
        assert!(!get_bookmarks().is_empty());
    }

    #[test]
    fn all_bookmark_paths_exist() {
        for bm in get_bookmarks() {
            assert!(
                Path::new(&bm.path).exists(),
                "bookmark path does not exist: {}",
                bm.path
            );
        }
    }
}
