//! # path_trav
//! Compare two paths to check if there are path traversal.
//! Useful for APIs that serve dynamic files.
//!
//! **Note:** this is a security tool. If you see something wrong, [open an issue in GitHub](https://github.com/gatomo-oficial/path_trav/issues)

use std::path::Path;
use substring::Substring;

/// ## is_path_traversal
/// Compare two routes to check if there are path traversal.
///
/// **Note:** this is a security tool. If you see something wrong, [open an issue in GitHub](https://github.com/gatomo-oficial/path_trav/issues)
///
/// ## Args
/// * `base`: `&Path` - Root path, the base to compare
/// * `relative`: `&Path` - Relative path that will be checked
///
/// ## Returns
/// `Result<bool, &'static str>`
///
/// ## Examples
/// ```
/// use std::path::Path;
/// use path_trav::is_path_trav;
///
/// let important_file = Path::new("./data/../../../etc/passwd");
///
/// // with absolute path
/// is_path_trav(&Path::new("/home/user/data"), &important_file).unwrap();
///
/// // with relative path
/// is_path_trav(&Path::new("./data"), &important_file).unwrap();
/// ```

pub fn is_path_trav(base: &Path, relative: &Path) -> Result<bool, &'static str> {
    if !base.exists() {
        return Err("Base path doesn't exist");
    }

    if !relative.exists() {
        return Err("Relative path doesn't exist");
    }

    let path1_absolute = base.canonicalize().unwrap().to_str().unwrap().to_string();

    let path2_absolute = relative
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let path2_split = path2_absolute.substring(0, path1_absolute.len());

    Ok(!path2_split.eq(&path1_absolute))
}
