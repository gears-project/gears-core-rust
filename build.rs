extern crate peg;

fn main() {
    peg::cargo_build("src/flox_grammar.rustpeg");
    peg::cargo_build("src/dsl/command_grammar.rustpeg");
}
