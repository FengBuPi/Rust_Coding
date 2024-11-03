fn main() {
    let mut game = Game::new();
    game.add();
    println!("{}", game.get_count());
}
struct Game {
    count: i32,
}
impl Game {
    fn new() -> Game {
        let game = Game { count: 0 };
        game
    }

    fn handle_add(&self) {
        let cb = || self.add();
        count(cb)
    }

    fn add(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

fn count(f: impl Fn() -> i32) {
    f()
}
