// fn main() {
//     let a = Box::new(3);
//     println!("a = {}", a); // a = 3

//     // 对Box包裹的数据进行解引用,即可获得该值
//     let b = *a + 1;
//     println!("a = {}", b);
// }
fn main() {
    let s = Box::new(String::from("hello world"));
    display(&s)
}

fn display(s: &str) {
    println!("{}", s);
}
