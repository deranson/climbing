mod modules;

use modules::strings::qstring;
use modules::variables_mutability;
fn main() {
    let quote = qstring("Learning rust language is tough for me, so record practice here");

    println!("{}", quote);

    variables_mutability::create_variable();
}
