// 普通函数
fn count1(a: i32, b: i32) -> i32 {
    a + b
}

// 普通函数中创建闭包函数
fn count2(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

// 普通函数传入闭包函数
fn count3(a: i32, f: impl Fn(i32) -> i32) -> i32 {
    f(a)
}

fn main() {
    // 1
    assert_eq!(count1(1, 2), 3);
    println!("{}", count1(1, 2));

    // 2
    assert_eq!(count2(1)(2), 3);
    println!("{}", count2(1)(2));

    // 3
    assert_eq!(count3(1, |b| b + 2), 3);
    println!("{}", count3(1, |b| b + 2));
    // 或者
    let cb = |b| b + 2;
    assert_eq!(count3(1, cb), 3);
    println!("{}", count3(1, count2(2)));
}
