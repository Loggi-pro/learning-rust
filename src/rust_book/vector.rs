#[allow(dead_code)]
pub fn run() {
    let v: Vec<i32> = Vec::new();
    println!("Vector without values {:#?}", v);
    let mut v = vec![1, 2, 3];
    println!("Vector with values {:?}", v);
    println!("Update vector");
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    println!("Vector after update {:?}", v);

    //
    let third = &v[2];
    println!("Third element of vector is {}", third);
    println!("Get element of vector by match syntax");
    match v.get(2) {
        Some(a) => println!("Third element of vector is {} (match)", a),
        None => println!("There is no third element"),
    }
    println!("Iterating: ");
    for i in &v {
        print!("{} ", i);
    }
    println!("Iterating with mutation: ");
    for i in &mut v {
        *i += 50;
        print!("{} ", i);
    }
    println!("Universal types in vector:");
    let row = vec![
        SpreasheetCell::Int(3),
        SpreasheetCell::Text(String::from("blue")),
        SpreasheetCell::Float(10.12),
    ];
    println!("{:?}", row);
    //
    println!("Iterate with index:");
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate(){
        println!("{} is at index {}",value,index);
    }
}

#[derive(Debug)]
enum SpreasheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
