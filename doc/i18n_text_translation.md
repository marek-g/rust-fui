# Text translation

This chapter shows how to localize your `fui` application with [`translation`](https://crates.io/crates/translation) crate that is based on [`tr!`](https://crates.io/crates/tr) macro.

Please note that text translation of `fui` application can be done with any external localisation crate. The `translate` crate is great if you like to have English text placed in the source code and be compatible with the [`gettext`](https://www.gnu.org/software/gettext/) library. If you like to keep only ids of text entries in your source code instead and use programmable translation files, you may be more interested in crates like [`fluent`](https://crates.io/crates/fluent).

This is a summary taken from the [`translation`](https://crates.io/crates/translation) crate. For more information and the latest documentation please refer to the crate's documentation.

## Minimal example:

```rust
use translation::{tr_init, tr};

#[derive(rust_embed::RustEmbed)]
#[folder = "i18n/mo"]
struct Translations;

fn main() {
    tr_init!("locale", Translations);

    println!("{}", tr!("Hello, world!"));
}
```

The files from `i18n/mo` source folder are embedded in the executable file. The `tr_init!` macro looks for (in the following order):
- `locale/{lang}/LC_MESSAGES/{module_name}.mo` in the file system
- `locale/{lang}/{module_name}.mo` in the file system
- `{lang}/LC_MESSAGES/{module_name}.mo` in the embedded `Translations` struct
- `{lang}/{module_name}.mo` in the embedded `Translations` struct  

The `locale` folder is looked for relative to the application executable file.

For more examples how to use `tr!` macro (arguments, plurals, context etc.) please refer to [`tr`](https://crates.io/crates/tr) crate documentation.

## Usage

_Note. This instruction uses `cargo-i18` tool. Please also read https://github.com/kellpossible/cargo-i18n for more information._ 

### Configuration Steps

Install required tools:

```shell script
cargo install xtr
cargo install cargo-i18n
```

Add the following to your `Cargo.toml` dependencies:

```toml
translation = "1"
```

Create an `i18n.toml` file in the root directory of your crate:
 
 ```toml
fallback_language = "en"

[gettext]
target_languages = ["pl"]
output_dir = "i18n"
```
 
Run `cargo i18n` tool:

```shell script
cargo i18n
``` 

It scans the code and creates and updates the localization files. You can run it every time you want to update your localization files or compile `po` files.
