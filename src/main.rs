mod modules;

use modules::strings::qstring;

fn main() {
    let quote = qstring("Learning rust language is tough for me, so record practice here");

    println!("{}", quote);

}
