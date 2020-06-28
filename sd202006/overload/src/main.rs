struct DummyType;

trait MyAdd<T, S> {
    fn myadd(x: T) -> S;
}

impl MyAdd<i32, i32> for DummyType {
    fn myadd(x: i32) -> i32 {
        x
    }
}

impl MyAdd<(i32, i32), i32> for DummyType {
    fn myadd(x: (i32, i32)) -> i32 {
        x.0 + x.1
    }
}

fn main() {
    println!("{}", DummyType::myadd(1));
    println!("{}", DummyType::myadd((1, 3)));
}
