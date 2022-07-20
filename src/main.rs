use crap_regex::parse;

fn main() {
    let reg = parse("aa!bc");
    // println!("\n{:?}\n", reg);
    reg.print();
}
