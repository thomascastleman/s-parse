#[macro_use] extern crate lazy_static;
extern crate regex;
mod parse;

fn main() {
    let s = "( (f x) a   (f (* 10 2) x)   c   d  )";
    println!("parsed \"{}\" as {:?}", s, parse::parse(&s));
}