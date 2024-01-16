use helix_stdx::path::{canonicalize, fold_home_dir, normalize};

use std::path::{Path, PathBuf};

pub fn get_canonicalized_path(path: &Path) -> PathBuf {
    canonicalize(path, helix_loader::current_working_dir())
}

pub fn get_relative_path(path: &Path) -> PathBuf {
    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        let cwdir = normalize(helix_loader::current_working_dir());
        normalize(&path)
            .strip_prefix(cwdir)
            .map(PathBuf::from)
            .unwrap_or(path)
    } else {
        path
    };
    fold_home_dir(&path)
}

/// Returns a truncated filepath where the basepart of the path is reduced to the first
/// char of the folder and the whole filename appended.
///
/// Also strip the current working directory from the beginning of the path.
/// Note that this function does not check if the truncated path is unambiguous.
///
/// ```   
///    use helix_core::path::get_truncated_path;
///    use std::path::Path;
///
///    assert_eq!(
///         get_truncated_path("/home/cnorris/documents/jokes.txt").as_path(),
///         Path::new("/h/c/d/jokes.txt")
///     );
///     assert_eq!(
///         get_truncated_path("jokes.txt").as_path(),
///         Path::new("jokes.txt")
///     );
///     assert_eq!(
///         get_truncated_path("/jokes.txt").as_path(),
///         Path::new("/jokes.txt")
///     );
///     assert_eq!(
///         get_truncated_path("/h/c/d/jokes.txt").as_path(),
///         Path::new("/h/c/d/jokes.txt")
///     );
///     assert_eq!(get_truncated_path("").as_path(), Path::new(""));
/// ```
///
pub fn get_truncated_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let cwd = helix_loader::current_working_dir();
    let path = path
        .as_ref()
        .strip_prefix(cwd)
        .unwrap_or_else(|_| path.as_ref());
    let file = path.file_name().unwrap_or_default();
    let base = path.parent().unwrap_or_else(|| Path::new(""));
    let mut ret = PathBuf::new();
    for d in base {
        ret.push(
            d.to_string_lossy()
                .chars()
                .next()
                .unwrap_or_default()
                .to_string(),
        );
    }
    ret.push(file);
    ret
}
