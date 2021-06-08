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

## Usage

main.rs
```
mod chapters;

use chapters::strings;

fn main() {
    ...
}
```
chapters.rs
```
pub mod strings;

...
```
chapters/strings.rs
```
/// Create String from &str directly

pub fn qstring(str: &str) -> String {
    String::from(str)
}

...
```

