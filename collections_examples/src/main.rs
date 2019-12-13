use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1,2,3,4,5];

    let first = &v[0];
    println!("{}", first);

    v.push(6);

    for i in &v {
        println!("{}", i);
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap();

    println!("score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
