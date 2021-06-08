# Climbing
Learning rust language is tough for me, so record practice here.

## Structure
```
src
 ┣ modules
 ┃ ┗ strings.rs
 ┃ ┗ variables_mutability.rs
 ┣ modules.rs
 ┗ main.rs
```

main.rs
```
mod modules;

use modules::strings::qstring;

fn main() {
    let quote = qstring("Learning rust language is tough for me, so record practice here");

    println!("{}", quote);
}
...
```
modules.rs
```
pub mod strings;

...
```
modules/strings.rs
```
/// Create String from &str directly

pub fn qstring(str: &str) -> String {
    String::from(str)
}

...
```

## Usage

- Install rust in your local
- Clone this repository to your local
- Reference and use modules in `main.rs`
- Run `cargo run` command
