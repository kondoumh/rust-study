fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces is {}", spaces);

    let z = 2.0;
    let z: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'c';
    let cat = '😺';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);
    let five_hundred = tup.0;

    let ar = [1, 2, 3, 4, 5];
    let index = 4;
    let element = ar[index];

    println!("The value of element is: {}", element);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is {}", x);

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Tha value of number is: {}", number);

    let ar = [10, 20, 30, 40, 50];
    for element in ar.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!")
}

fn five() -> i32 {
    5
}
