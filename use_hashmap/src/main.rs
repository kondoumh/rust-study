use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}, {}", field_name, field_value); // error!

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score); // Some(10)


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", words);
}
