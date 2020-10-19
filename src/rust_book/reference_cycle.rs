use crate::rust_book::reference_cycle::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail (&self)->Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_,item)=> Some(item),
            Nil=>None
        }
    }
}

#[allow(dead_code)]
pub fn run() {
println!("Creating recurcive list");
//This code creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle. 
//There are println! statements along the way to show what the reference counts are at various points in this process.
let a = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
println!("a initial Rc count = {}",Rc::strong_count(&a));
println!("a next item = {:?}",a.tail());

let b = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
println!("a rc count after b creation = {}", Rc::strong_count(&a));
println!("b initial Rc count = {}",Rc::strong_count(&b));
println!("b next item = {:?}",b.tail());

if let Some(link) = a.tail(){
    *link.borrow_mut() = Rc::clone(&b);
}
println!("b Rc count after changing a = {}",Rc::strong_count(&b));
println!("a Rc count after changing a = {}",Rc::strong_count(&a));
// Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
//
println!("Using weak with Tree data structure to deal with referency cycle:");
let leaf = Rc::new(Node {
    value:3,
    parent:RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});
println!("leaf parent= {:?}",leaf.parent.borrow().upgrade());
let branch = Rc::new(Node {
    value:5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);
println!("leaf parent= {:?}",leaf.parent.borrow().upgrade());
//
println!("Visualizing strong_count and weak_count:");
let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

{
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch strong = {}, weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));

    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
}

println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
}

//tree node
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, //if this node is dropped, it will not affect parent
    children: RefCell<Vec<Rc<Node>>>, //This node is own its childen, and if it drops, it will drop all childerns
}
