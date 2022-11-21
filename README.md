# Path trav
A simple path traversal checker made with Rust. Useful for APIs that serve dynamic files.

**Note:** this is a security tool. If you see something wrong, [open an issue in GitHub](https://github.com/gatomo-oficial/path_trav/issues).

## How it works?
The `is_path_trav` function receives two paths, one is the base path and the other is the path to check.
To verify if the second is inside the first, `path_trav` turn paths into absolute and check if the second route contains the first.

#### Example 1.
> **Base&nbsp;&nbsp;:** */home/user/data* &nbsp;&nbsp;**-->**&nbsp; ***/home/user/data***

> **Rel&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;:** *./data/folder* &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**-->**&nbsp; ***/home/user/data**/folder*

Relative path is inside base path.

#### Example 2.
> **Base&nbsp;&nbsp;:** */home/user/data* &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**-->**&nbsp; */home/user/data*

> **Rel&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;:** *./data/../../../etc/passwd* &nbsp;&nbsp;**-->**&nbsp; */etc/passwd*

Relative paths isn't inside base path, tries to acces sensitive data

## Example
Add `path_trav` to your Cargo.toml
```toml
[dependencies]
path_trav = "1.0.0"
```

Then, on your main.rs file
```rust
use std::path::Path;
use path_trav::is_path_trav;

fn main() {
    let important_file = Path::new("./data/../../../etc/passwd");

    // with absolute path
    let check_abs = is_path_trav(&Path::new("/home/user/data"), &important_file).unwrap();

    // with relative path
    let check_rel = is_path_trav(&Path::new("./data"), &important_file).unwrap();
}
```
`is_path_trav` returns `Result<bool, &'static str>`. Unwrap it or use match to get the result. If returns true, there are path traversal.

## License
`path_trav` is licensed under the [GPL-3 license](https://www.gnu.org/licenses/gpl-3.0.html).

## Contribute
🥳 Any PR is welcome! Is a small project, so the guideline is to follow the code style and not make insane pruposes.

## Links
- [Web](https://gatomo.ga)
- [Donate (via PayPal)](https://paypal.me/gatomooficial)
- [Discord (spanish)](https://discord.gatomo.ga)

*Gátomo - GPL-3 License*
