// &str类型的数据是否存储在堆上还是静态内存中,取决于他所引用的数据类型是谁
// 如果是字符串字面量，则存储在静态内存中，如果是String 类型，则存储在堆上
fn main() {
    let literal: &str = "Hello, world!"; // 字符串字面量，存储在静态内存
    let heap_string = String::from("Hello, Rust!"); // String 类型，存储在堆上
    let slice: &str = &heap_string; // &str 引用，指向堆上的 String 数据

    println!("literal: {}", literal);
    println!("slice: {}", slice);
}
