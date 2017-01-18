use std::io;

mod lib;


fn main() {
    let code_path = std::env::args().nth(1).unwrap();
    lib::run(code_path, io::stdin(), &mut io::stdout())
}
