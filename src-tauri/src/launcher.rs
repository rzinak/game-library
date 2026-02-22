use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LaunchError {
    #[error("Executable not found: {0}")]
    ExecutableNotFound(String),
    #[error("Failed to spawn process: {0}")]
    SpawnFailed(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LaunchTarget {
    Steam { app_id: u32 },
    Executable { path: String },
}

impl LaunchTarget {
    pub fn steam(app_id: u32) -> Self {
        Self::Steam { app_id }
    }

    pub fn executable(path: impl Into<String>) -> Self {
        Self::Executable { path: path.into() }
    }

    /// Returns the Steam URI for a Steam target, or `None` for executables.
    pub fn steam_uri(&self) -> Option<String> {
        match self {
            Self::Steam { app_id } => Some(format!("steam://run/{}", app_id)),
            Self::Executable { .. } => None,
        }
    }
}

/// Launches the given target. For Steam games this opens the `steam://run/<id>` URI;
/// for custom games it delegates to [`spawn_executable`] (child is discarded).
pub fn launch(target: &LaunchTarget) -> Result<(), LaunchError> {
    match target {
        LaunchTarget::Steam { app_id } => launch_steam(*app_id),
        LaunchTarget::Executable { path } => {
            spawn_executable(path)?;
            Ok(())
        }
    }
}

/// Opens the Steam URI for the given app ID using the OS default handler.
pub fn launch_steam(app_id: u32) -> Result<(), LaunchError> {
    let uri = format!("steam://run/{}", app_id);
    log::info!("Launching Steam game: app_id={} uri={}", app_id, uri);
    open_uri(&uri)
}

/// Spawns the game at `path` and returns the child process handle when available.
///
/// On macOS, if `path` is a `.app` bundle directory the system `open` command is used
/// — which hands off to launchd — so no direct child handle is returned (`None`).
/// On all other platforms, or when `path` points to a regular executable, the process
/// is spawned directly and `Some(child)` is returned.
pub fn spawn_executable(path: &str) -> Result<Option<std::process::Child>, LaunchError> {
    if !Path::new(path).exists() {
        log::warn!("Executable not found: {}", path);
        return Err(LaunchError::ExecutableNotFound(path.to_string()));
    }

    #[cfg(target_os = "macos")]
    if Path::new(path).is_dir() && path.ends_with(".app") {
        log::info!("Launching macOS app bundle via open: {}", path);
        Command::new("open").arg(path).spawn()?;
        return Ok(None);
    }

    log::info!("Spawning executable: {}", path);
    Ok(Some(Command::new(path).spawn()?))
}

/// Resolves the process name that the OS will report for the given executable path.
///
/// For macOS `.app` bundles, this inspects `Contents/MacOS/` to find the actual
/// binary name (e.g. `"/Applications/Aseprite.app"` → `"aseprite"`).
/// For plain executables on all platforms, this is simply the file name.
pub fn resolve_process_name(exe_path: &str) -> String {
    let path = Path::new(exe_path);

    #[cfg(target_os = "macos")]
    if exe_path.ends_with(".app") {
        let macos_dir = path.join("Contents/MacOS");
        if let Ok(entries) = std::fs::read_dir(&macos_dir) {
            // Return the first non-hidden, non-directory entry
            let name = entries
                .flatten()
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    !name.starts_with('.') && e.file_type().map(|t| !t.is_dir()).unwrap_or(false)
                })
                .next()
                .map(|e| e.file_name().to_string_lossy().to_string());

            if let Some(n) = name {
                return n;
            }
        }
        // Fallback: bundle name without .app extension
        return path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
    }

    path.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Opens a URI using the platform's default handler.
fn open_uri(uri: &str) -> Result<(), LaunchError> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(uri).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(uri).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", "", uri]).spawn()?;
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        let _ = uri;
    }
    Ok(())
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    // --- LaunchTarget ---

    #[test]
    fn steam_uri_for_steam_target() {
        let target = LaunchTarget::steam(440);
        assert_eq!(target.steam_uri(), Some("steam://run/440".to_string()));
    }

    #[test]
    fn steam_uri_none_for_executable_target() {
        let target = LaunchTarget::executable("/usr/games/example");
        assert_eq!(target.steam_uri(), None);
    }

    #[test]
    fn executable_target_stores_path() {
        let target = LaunchTarget::executable("/games/hollow_knight");
        assert_eq!(
            target,
            LaunchTarget::Executable {
                path: "/games/hollow_knight".to_string()
            }
        );
    }

    #[test]
    fn steam_target_stores_app_id() {
        let target = LaunchTarget::steam(570);
        assert_eq!(target, LaunchTarget::Steam { app_id: 570 });
    }

    // --- spawn_executable ---

    #[test]
    fn spawn_nonexistent_returns_error() {
        let err = spawn_executable("/absolutely/does/not/exist.exe").unwrap_err();
        assert!(matches!(err, LaunchError::ExecutableNotFound(_)));
    }

    #[cfg(unix)]
    #[test]
    fn spawn_binary_returns_some_child() {
        if !Path::new("/usr/bin/true").exists() {
            return;
        }
        let child = spawn_executable("/usr/bin/true")
            .expect("should not error")
            .expect("direct binary should give Some(child)");
        drop(child); // let it clean up
    }

    #[cfg(unix)]
    #[test]
    fn child_is_running_then_killed() {
        let mut child = std::process::Command::new("/bin/sleep")
            .arg("60")
            .spawn()
            .expect("spawn failed");

        assert!(child.try_wait().unwrap().is_none(), "should be running");

        child.kill().expect("kill failed");
        let status = child.wait().expect("wait failed");
        assert!(!status.success());
    }

    // --- resolve_process_name ---

    #[test]
    fn process_name_from_plain_binary() {
        assert_eq!(resolve_process_name("/usr/bin/aseprite"), "aseprite");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn process_name_from_windows_exe() {
        assert_eq!(
            resolve_process_name("C:\\Games\\aseprite.exe"),
            "aseprite.exe"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn process_name_from_app_bundle_falls_back_to_stem_when_no_contents() {
        // A path that ends in .app but has no Contents/MacOS — falls back to bundle stem
        assert_eq!(
            resolve_process_name("/Applications/FakeGame.app"),
            "FakeGame"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn process_name_from_real_app_bundle_if_present() {
        // Only runs when Aseprite (or another .app) is actually installed
        let path = "/Applications/Aseprite.app";
        if Path::new(path).exists() {
            let name = resolve_process_name(path);
            assert!(!name.is_empty(), "should return a non-empty process name");
        }
    }
}
