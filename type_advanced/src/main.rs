type Kilometers = i32;

type Thunk = Box<Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);
}

fn takes_long_type(f: Thunk) {

}