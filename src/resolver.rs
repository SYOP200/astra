use std::{
    env,
    path::{Path, PathBuf},
};

/// Resolve an executable using the current PATH.
///
/// Returns the full path to the executable if found.
pub fn resolve(program: &str) -> Option<PathBuf> {
    // Already a path?
    if program.contains('/') {
        let path = PathBuf::from(program);

        if is_executable(&path) {
            return Some(path);
        }

        return None;
    }

    let path = env::var_os("PATH")?;

    for directory in env::split_paths(&path) {
        let candidate = directory.join(program);

        if is_executable(&candidate) {
            return Some(candidate);
        }
    }

    None
}

/// Returns true if the file exists.
///
/// (We'll improve this later to check execute permissions.)
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
