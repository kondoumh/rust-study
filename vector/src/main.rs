enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v:Vec<i32> = Vec::new();

    let v = vec![1, 2, 3, 4];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 100;
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
