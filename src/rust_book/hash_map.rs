use std::collections::HashMap;
#[allow(dead_code)]
pub fn run() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    println!("Hash  map: {:?}", scores);
    //
    println!("Create hash map from vectors");
    let teams = vec![String::from("green"), String::from("yellow")];
    let initial_scores = vec![10, 20];
    let mut scores2: HashMap<_, _> = //underscores=>auto derive type from type of vectors
    teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Hash  map: {:?}", scores2);
    //Hashmap takes ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("Hash  map takes ownership: {:?}", map);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    let team_name = String::from("green");
    let score = scores2.get(&team_name);
    match score {
        Some(a) => println!("Item by key {:?} is {}", team_name, a),
        None => println!("There is no item associated with key {}", team_name),
    }
    //
    println!("Iterating over map:");
    for (key, value) in &scores2 {
        println!("{}:{}", key, value);
    }
    //
    println!("Iterating over map:");
    scores2.insert(String::from("green"), 100);
    println!("{:?}", scores2);
    //check if key exist
    scores2.entry(String::from("Yellow")).or_insert(50);
    //
    println!("Counting words in text");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
