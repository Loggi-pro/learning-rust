/*
Iterator trait looks like this
pub trait Iterator {
    type Item;
    fn next(&mut self)-> Option<Self::Item>;
    //methods with default implementation elided
}
*/
#[allow(dead_code)]
pub fn run() {
    println!("Iterate throught vector by iterator ");
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Got: {}",val);
    }
    //iterator demo
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
    //iterator sum
    println!("Sum method consume iterator:");
    let v1_iter = v1.iter();
    let total:u32 = v1_iter.sum();
    println!("Summ of vector is = {}",total);
    println!("Iterator after that line is not usable");
    //map
    println!("Map vector:");
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("Got v2 vector: {:?}",v2);
    //
    println!("Using filter:");
    let shoes = vec![
        Shoe{size:10,style:String::from("sneaker")}, 
        Shoe{size:13, style:String::from("sandal")},
        Shoe{size:10,style: String::from("boot")}
    ];
    let in_my_size = shoes_in_my_size(shoes,10);
    println!("List of shoes in my size: {:?}",in_my_size);
    //
    println!("Implement own interator that can count from 1 to 5");
    let mut counter = Counter::new();
    println!("Next value is {:?}",counter.next());
    println!("Next value is {:?}",counter.next());
    println!("Next value is {:?}",counter.next());
    println!("Next value is {:?}",counter.next());
    println!("Next value is {:?}",counter.next());
    println!("Next value is {:?}",counter.next());
    //
    let sum:u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a,b)| a*b)
    .filter(|x|x%3 ==0)
    .sum();
    println!("This expresion is equal to {:?} (must be 18)",sum);
    //
    println!("Example of pair iterator");
    let mut it = PairIterator::new(&v1);
    println!("First pair: {:?}",it.next());
    println!("Second pair: {:?}",it.next());
    println!("Third pair: {:?}",it.next());

}

//filter shoes
#[derive(PartialEq,Debug)]
struct Shoe {
    size:u32,
    style:String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size==shoe_size).collect() //into_iter takes ownership of vector
}


//implement own interator that can count from 1 to 5
struct Counter {
    count:u32
}
impl Counter {
    fn new()->Counter {
        Counter {count:0}
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) ->Option<Self::Item>{
        if self.count <5 {
            self.count +=1;
            Some(self.count)
        } else {None}
    }
}

//Pair iterator
struct PairIterator<'a,T> where T:Copy {
    vec:&'a Vec<T>,
    current_pos:usize,
}

impl<'a,T> PairIterator<'a,T> where T:Copy{
    fn new(vec:&'a Vec<T>)->PairIterator<'a,T>{
        PairIterator {vec:vec,current_pos:0}
    }
}
#[derive(Debug)]
struct Pair<T> where T:Copy {
    first:T,
    second:T,
}

impl<'a,T> Iterator for PairIterator<'a,T> where T:Copy {
    type Item = Pair<T>;
    fn next(&mut self) ->Option<Self::Item>{
    if self.current_pos+1<self.vec.len() {
        self.current_pos = self.current_pos+1;
        Some(Pair{first:self.vec[self.current_pos-1],second:self.vec[self.current_pos]})
    }   else {
        None
    }
    }
}