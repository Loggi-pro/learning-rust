use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
//memoization of closure result
struct Cacher<T,Tin,Tout>
where T:Fn(Tin)->Tout,
     Tin:Copy+Hash+Eq,
     Tout:Copy
{
    calculation: T,
    value: HashMap<Tin,Tout>
}


impl <T,Tin,Tout> Cacher<T,Tin,Tout> 
where   T: Fn(Tin)->Tout,
        Tin:Copy+Hash+Eq,
        Tout:Copy
{

    fn new(calculation:T)->Cacher<T,Tin,Tout>{
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
fn value(&mut self,arg:Tin)->Tout{
    match self.value.get(&arg) {
        Some(v)=>*v,
        None =>{
            let v = (self.calculation)(arg);
            self.value.insert(arg,v);
            v
        }
    }
}

}


fn generate_workout(insensity:u32,random_number:u32){
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if insensity < 25 {
        println!("Today, do {} pushaps",expensive_result.value(insensity));
        println!("Next, do {} situps",expensive_result.value(insensity));
    }else {
        if random_number ==3 {
            println!("Take a break today!");
        } else {
            println!("Today, run for {} minutes", expensive_result.value(insensity));
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Using memoization to calc some algotithm");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value,simulated_random_number);
    //
    println!("Using closures which capture content:");
    let x = 4;
    let equal_to_x = |z| z==x;
    let y = 4;
    let not = if equal_to_x(y) {""} else {"NOT"};
    println!("Variable y ({}) is {} equal to captured x(4).",y,not);
     //
    println!("Using move keyword with closures:");
    let x = vec![1,2,3];
    let equal_to_x_vec = move |z| z ==x;
    println!("Can't use x here because it was moved to closure (keyword 'move')");
    let y = vec![1,2,3];
    let not = if equal_to_x_vec(y) {""} else {"NOT"};
    println!("Vector y is {} equal to captured vector x.",not);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
#[test]
fn call_with_different_type() {
    let mut c = Cacher::new(|a:&String| a.len());

    let v1 = c.value(&String::from("Test string"));

    assert_eq!(v1, 11);
}

