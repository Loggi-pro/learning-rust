/* annotation syntax
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/
#[allow(dead_code)]
pub fn run() {
    println!("Lifetime of the function result:");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
    //proof
    println!("Check normal:");
    this_will_compile();
    //check 2
    println!("Check bad:");
    this_will_not_compile();
    //
    println!("Test lifetime of structs:");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Struct with reference: {:?}", i);
    //struct cannot go out of scope before first_sentence.
    //
    println!("Lifetime elision:");
    println!("1 rule: Each reference parameter get one lifetime parameter");
    println!("2 rule: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters");
    println!("3 rule: If there are multiple input lifetime parameters, but one of them is &self or &mut self (method), the lifetime of self is assigned to all output lifetime parameters");
    let word = first_word(&novel);
    println!("First word is {}", word);
    //
    println!("Method lifetime:");
    println!("Call method with explicit lifetime. Level is {}", i.level());
    //
    println!("Static lifetime:");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    //
    println!("Generic function with trait and lifetime:");
    longest_with_an_announcement(string1.as_str(), string2, "test");
}
//explicit lifetime for rust compiler to know that any of this string may be borrowed
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //lifetime of the returned reference  is the same
    //as the smaller of the lifetimes of the references passed in.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//chekc longest function bad
fn this_will_not_compile() {
    println!("This will not compile!");
    /* let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);*/
}
//chekc longest function good
fn this_will_compile() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

//structs
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//dont need explicit lifetime
//lifetime elision rules:
//1 rule: Each reference parameter get one lifetime parameter
//fn foo<'a>(x: &'a i32); fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
//2 rule: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
//fn foo<'a>(x: &'a i32) -> &'a i32
//3 rule: If there are multiple input lifetime parameters, but one of them is &self or &mut self (method), the lifetime of self is assigned to all output lifetime parameters
// Here applied 1 and 2 rules->and result lifetime is derived
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] //full string
}

//lifetime for method
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

//all in one
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
