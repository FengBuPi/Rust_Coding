/**
 * 三种闭包形式的区别
 * 1. FnOnce 闭包可以访问其环境一次，它可能消费环境中的值，或者只是实现 Copy，以便在闭包中多次使用其捕获的变量。
 * 2. FnMut 闭包可以多次修改其捕获的变量的值，但不能访问环境中的值。
 * 3. Fn 闭包只能读取其环境中的值，它不能修改其捕获的变量的值，也不能访问环境中的值。
 *
 * 实现哪种 Fn 特征,取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们
 *
 * 闭包的类型推断
 * 闭包的类型推断与函数的类型推断类似，但闭包的类型推断更严格，因为闭包可以捕获其环境中的值。
 *
 * 闭包的生命周期
 * 闭包的生命周期与函数的生命周期类似，但闭包的生命周期更长，因为它可以捕获其环境中的值。
 *
 * 闭包的实现原理
 * 闭包的实现原理与函数的实现原理类似，但闭包的实现原理更复杂，因为它可以捕获其环境中的值。
 */
fn main() {
    let s = String::from("我是你爹");

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}
