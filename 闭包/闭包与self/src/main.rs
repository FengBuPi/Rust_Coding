/// 闭包与self
/// 在结构体中实现传递闭包参数 与 在全局作用域实现传递闭包参数

/// 在结构体中使用闭包时，需要使用self关键字，因为闭包会捕获到结构体的变量，
/// 而结构体本身并不拥有这些变量的所有权，因此需要使用self关键字来获取这些变量。'

fn main() {
    let mut game = Game::new();
    game.handle_add();
    println!("{}", game.get_count());
}

struct Game {
    count: i32,
}

impl Game {
    fn new() -> Game {
        let game = Game { count: 0 };
        game.preview();
        game
    }

    // 结构体下的普通回调函数(传入结构体实例)
    fn self_callback_function(&mut self, f: impl FnOnce(&mut Self) -> i32) -> i32 {
        f(self)
    }

    // 结构体下的普通回调函数(不传入结构体实例)
    fn noself_callback_function(f: impl FnOnce() -> i32) -> i32 {
        f()
    }

    // 调用其中的回调函数
    fn handle_add(&mut self) {
        // 第一种情况:
        // 结构体下的普通回调函数(传入结构体实例)的情况下的执行
        // let cb = |game: &mut Game| -> i32 {
        //     game.add_count();
        //     game.get_count()
        // };
        // self.count = self.self_callback_function(cb);

        // 第二种情况:
        // 结构体下的普通回调函数(不传入结构体实例)的情况下的执行
        let cb = || -> i32 {
            self.add_count();
            self.get_count()
        };
        self.count = Game::noself_callback_function(cb);

        // 第三种情况:
        // let cb = || -> i32 {
        //     self.add_count();
        //     self.get_count()
        // };
        // 全局作用域下的普通回调函数的情况下的执行
        // self.count = normal_callback_function(cb);
    }

    fn preview(&self) {
        println!("self.count = {}", self.count);
    }

    fn add_count(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

// 全局作用域下的普通回调函数
fn normal_callback_function(mut f: impl FnMut() -> i32) -> i32 {
    f()
}
