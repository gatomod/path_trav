//! # path_trav
//! Compare two paths to check if there are path traversal.
//! Useful for APIs that serve dynamic files.
//!
//! **Note:** this is a security tool. If you see something wrong, [open an issue in GitHub](https://github.com/gatomo-oficial/path_trav/issues)
//!
//! ## Examples
//! For examples, see `PathTrav` trait
//!
//! ## Tests
//! There are a few integration tests where you can check the Path Trav behavior.
//!

use std::{io::ErrorKind, path::Path};
use substring::Substring;

/// ## PathTrav trait
/// Trait that extends `std::path::Path`
pub trait PathTrav {
    /// ## is_path_trav
    /// Compare two paths to check if there are path traversal.
    ///
    /// **Note:** this is a security tool. If you see something wrong, [open an issue in GitHub](https://github.com/gatomo-oficial/path_trav/issues)
    ///
    /// ## Args
    /// * `&self`: `&Path` - Root path, the base to compare (given by `std::path`)
    /// * `rel`: `&Path` - Relative path that will be checked
    ///
    /// ## Returns
    /// `Result<bool, std::io::ErrorKind>`
    ///
    /// ## Examples
    /// ### With std::path::Path
    /// ```
    /// use std::{io::ErrorKind, path::Path};
    /// use path_trav::*;
    ///
    /// let server_folder       = Path::new("./");
    /// let server_file         = Path::new("./tests/test.rs");
    /// let important_file      = Path::new("~/../../etc/passwd");
    /// let non_existent_file   = Path::new("../weird_file");
    ///
    /// // Path is inside server_folder (Ok)
    /// assert_eq!(Ok(false), server_folder.is_path_trav(&server_file));
    ///
    /// // Path tries to acces sensitive data (Path Traversal detected)
    /// assert_eq!(Ok(true), server_folder.is_path_trav(&important_file));
    ///
    /// // File does not exists (ENOENT)
    /// assert_eq!(Err(ErrorKind::NotFound), server_folder.is_path_trav(&non_existent_file));
    /// ```
    ///
    /// ### With std::path::PathBuf
    ///```
    /// use std::{io::ErrorKind, path::PathBuf};
    /// use path_trav::*;
    ///
    /// let server_folder       = PathBuf::from("./");
    /// let server_file         = PathBuf::from("./tests/test.rs");
    /// let important_file      = PathBuf::from("~/../../etc/passwd");
    /// let non_existent_file   = PathBuf::from("../weird_file");
    ///
    /// // Path is inside server_folder (Ok)
    /// assert_eq!(Ok(false), server_folder.is_path_trav(&server_file));
    ///
    /// // Path tries to acces sensitive data (Path Traversal detected)
    /// assert_eq!(Ok(true), server_folder.is_path_trav(&important_file));
    ///
    /// // File does not exists (ENOENT)
    /// assert_eq!(Err(ErrorKind::NotFound), server_folder.is_path_trav(&non_existent_file));
    /// ```
    ///

    fn is_path_trav(&self, rel: &Path) -> Result<bool, ErrorKind>;
}

impl PathTrav for std::path::Path {
    fn is_path_trav(&self, rel: &Path) -> Result<bool, ErrorKind> {
        let base_abs = match self.canonicalize() {
            Err(err) => return Err(err.kind()),
            Ok(data) => data,
        };

        let base_abs = match base_abs.to_str() {
            None => return Err(ErrorKind::InvalidData),
            Some(da) => da,
        };

        let rel_abs = match rel.canonicalize() {
            Err(err) => return Err(err.kind()),
            Ok(data) => data,
        };

        let rel_abs = match rel_abs.to_str() {
            None => return Err(ErrorKind::InvalidData),
            Some(da) => da,
        };

        let trimmed_rel_abs = rel_abs.substring(0, base_abs.len());

        Ok(!trimmed_rel_abs.eq(base_abs))
    }
}
