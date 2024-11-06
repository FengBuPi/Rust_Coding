/ 使用Cell的前提是实现了Copuy trait的数据
use std::cell::Cell;
fn main() {
    // 没使用cell智能指针之前时,一个变量只能被一个引用所拥有,其中要么只有一个可变借用，要么是多个不可变借用
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);

    // 使用了cell智能指针之后,一个变量可以有多个可变借用
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());
}
