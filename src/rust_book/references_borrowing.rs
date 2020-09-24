#[allow(dead_code)]
pub fn run() {
    let s = String::from("hello");
    let len = calc_len(&s);

    println!("The length of '{}' is {}.", s, len);
    //mutable references
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The new string is '{}'", s2);
    //its not permissed to use several mutable references simultinously
    {
        //create this ref in new scope
        let _r1 = &mut s2;
    } //must be end of scope here, ref is destroyed
    let _r2 = &mut s2;
}

//s is immutable
fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
