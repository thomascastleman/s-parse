mod parse;

fn main() {
    let s = "-5.182 x y)";
    println!("parsed \"{}\" as {:?}", s, parse::num_parse(&s));
}