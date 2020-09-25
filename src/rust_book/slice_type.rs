#[allow(dead_code)]
pub fn run() {
    let s = String::from("hello world");
    let word = first_word(&s);
    //s.clear(); //word still has the values 5 here, but there's no more string that
    //==============================================
    //we can make index part of string, it is called slice- reference to part of String
    let _hello = &s[0..5]; //[start_index, ending_index) (as iterator -> reference to one more that the last position)
    let _world = &s[6..11];
    let _hello_same = &s[..5]; //0 can be ommited
    let _world_same = &[6..]; //end can be ommited if it is last
    let _s_same = &[..]; //this is same as reference to a full string
    let word2 = first_word2(&s);
    println!("First world last index = '{}' and '{}'", word, word2);
    //
    //=======Slices for literals===================
    //after s.clear() you cannot use word2 anymore (compile error)
    let my_string_literal = "hello world";
    let word = first_word_for_literal_also(my_string_literal);
    println!("First world of string literal is '{}'", word);
    //=======Clices of array======================
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!(
        "First item of slice of integers is '{}' (must be '2')",
        slice[0]
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_for_literal_also(s: &str) -> &str {
    //function can take slices and strings
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
