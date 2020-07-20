#[macro_use] extern crate lazy_static;
extern crate regex;
mod parse;

fn main() {
    // an example usage
    let s = "((lambda (x) (* x x)) 50)";
    println!("Parsed \"{}\" as {:?}", s, parse::parse(&s));
}