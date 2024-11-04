use std::cell::Cell;
fn main() {
    let a = Cell::new("asdf"); // 注意此处没有使用mut 关键字,但是下面使用了的
    let b = a.get();
    a.set("qwer");
    let c = a.get();
    println!("{},{}", b, c);
}

// fn main() {
//     let mut a = "asdf"; // 创建一个可变字符串切片,必须使用mut 才能让这个变量修改
//     let b = a; // 将 a 的值赋给 b
//     a = "qwer"; // 修改 a 的值
//     let c = a; // 将修改后的 a 的值赋给 c

//     println!("{}, {}", b, c); // 打印 b 和 c 的值
// }
