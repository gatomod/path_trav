# Path trav

### A simple path traversal checker made with Rust. Useful for APIs that serve dynamic files.

<br />

[<img alt="license" src="https://img.shields.io/github/license/gatomo-oficial/path_trav.svg?color=06b6d4&style=for-the-badge&logo=Apache">](https://www.apache.org/licenses/LICENSE-2.0)
[<img alt="crates.io" src="https://img.shields.io/crates/v/path_trav.svg?style=for-the-badge&color=fc8d62&logo=rust">](https://crates.io/crates/path_trav)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-path_trav-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs">](https://docs.rs/path_trav)

<br />

**Note:** this is a security tool. If you see something wrong, [please, read the security policy](https://github.com/gatomod/path_trav/security).

## How it works?

The `is_path_trav` function is implemented in `std::path::Path`. It receives two paths, the base path and the path to check.
To verify if the second is inside the first, `path_trav` turn paths into absolute and check if the second route contains the first.

#### Example 1.

> **Base&nbsp;&nbsp;:** _/home/user/data_ &nbsp;&nbsp;**-->**&nbsp; **_/home/user/data_**

> **Rel&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;:** _./data/folder_ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**-->**&nbsp; **\*/home/user/data**/folder\*

Relative path is inside base path.

#### Example 2.

> **Base&nbsp;&nbsp;:** _/home/user/data_ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**-->**&nbsp; _/home/user/data_

> **Rel&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;:** _./data/../../../etc/passwd_ &nbsp;&nbsp;**-->**&nbsp; _/etc/passwd_

Relative path isn't inside base path, it's trying to access sensitive data

## Examples

First, add `path_trav` to your Cargo.toml

```toml
[dependencies]
path_trav = "2"
```

Then, on your main.rs file

```rust
use std::path::Path;
use path_trav::*;

fn main() {
    let server_folder       = Path::new("./");
    let server_file         = Path::new("./tests/test.rs");
    let important_file      = Path::new("~/../../etc/passwd");
    let non_existent_file   = Path::new("../weird_file");

    // Path is inside server_folder (Ok)
    assert_eq!(Ok(false), server_folder.is_path_trav(&server_file));

    // Path tries to access sensitive data (Path Traversal detected)
    assert_eq!(Ok(true), server_folder.is_path_trav(&important_file));

    // File does not exists (ENOENT)
    assert_eq!(Err(ErrorKind::NotFound), server_folder.is_path_trav(&non_existent_file));
}

```

`is_path_trav` returns `Result<bool, std::io::ErrorKind>`. If returns true, there are path traversal.

**Note:** _You can use it with `PathBuf`_

```rust
use std::path:PathBuf

let server_folder   = PathBuf::from("./");
let server_file     = PathBuf::from("./tests/test.rs");

assert_eq!(Ok(false), server_folder.is_path_trav(&server_file));
```

## Tests

There are a few integration tests in `/tests` folder where you can check the Path Trav behavior.

## License

`path_trav` is licensed under the [Apache 2.0 license](https://www.apache.org/licenses/LICENSE-2.0).

## Contribute

🥳 Any PR is welcome! Path Trav is a small project, so please follow the code style and avoid making insane proposals.

_Gátomo - Apache 2.0 License_
