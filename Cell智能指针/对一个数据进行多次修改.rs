// use std::cell::Cell;
fn main() {
    // let c = Cell::new("asdf");
    // let one = c.get();
    // c.set("qwer");
    // let two = c.get();
    // println!("{},{}", one, two);

    let c: &str = "asdf";
    let one = c;
    let one: &str = "qwer";
    println!("{},{}", c, one);
}
