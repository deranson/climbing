# Climbing
Learning rust language is tough for me, so record practice here.

## Structure
```
 climbing/
 ┣ docs
 ┃ ┗ strings.md
 ┃
 ┣ src
 ┃ ┗ modules
 ┃   ┗ strings.rs
 ┃   ┗ variables_mutability.rs
 ┃
 ┃ ┗ modules.rs
 ┃ ┗ main.rs
```

<details>
 <summary>main.rs</summary>
 
 ```
mod modules;

use modules::strings::qstring;

fn main() {
    let quote = qstring("Learning rust language is tough for me, so record practice here");

    println!("{}", quote);
}
...
```
</details>

<details>
 <summary>modules.rs</summary>
 
```
pub mod strings;

...
```
</details>

<details>
 <summary>modules/strings.rs</summary>
 
```
/// Create String from &str directly

pub fn qstring(str: &str) -> String {
    String::from(str)
}

...
```
</details>

## Usage

- Install rust in your local follow this [guide](https://www.rust-lang.org/tools/install)
- Clone this repository to your local
- Reference and use modules in `main.rs`
- Cd the root dir
- Run `cargo run` command

## Docs
Example: docs/foo.md is the note of the code src/modules/foo.rs of the content foo in Rust lang
### **Contents** & **Docs** & **Modules**
| Contents | Docs | Modules |
| --- | --- | --- |
| String | [strings.md](docs/strings.md) | [strings.rs](src/modules/strings.rs) |


## License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/deranson/climbing/blob/main/LICENSE)
