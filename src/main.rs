mod read_class;
mod class;
mod utils;
mod operation;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let bytecode = read_class::parse(filename.to_string());
    operation::execute(bytecode);
}
