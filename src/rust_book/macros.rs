#[macro_export]
macro_rules! make_vec {
    //body similar to match expression (arm pattern)
    ($($x:expr),*)=>{   // '$($x:expr)' get rust expression and give it a name 'x'
                        //',*' - zero or multiple args
                        //x->is every argument
        {
            let mut temp_vec = Vec::new();
            $( //generate multiple times (for each x)
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
//
use hello_macro::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[allow(dead_code)]
pub fn run() {
    println!("More info about how to write macros here: https://danielkeep.github.io/tlborm/book/index.html");
    println!("Declare own macros for vector:");
    let v:Vec<u32> = make_vec![1,2,3]; //$x pattern matches three times with the three expressions 1, 2, and 3.
    println!("   declared vector:{:?}",v);
    println!("Procedural macros for attributes: get some code as an input, operate on that code and produce some code as an output");
    Pancakes::hello_macro();
    println!("Attribute like macro(same as procedural): allow to create new attributes");
    println!("Function-like macro can take unknown number of arguments");
}
