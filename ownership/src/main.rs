fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}