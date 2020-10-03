fn max<T: PartialEq + Copy + PartialOrd>(list: &[T]) -> Option<T> {
    let mut largest = list.get(0)?;
    println!("Find max of ints");
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(*largest)
}

#[allow(dead_code)]
pub fn run() {
    //
    println!("Generic type: point");
    let p = Point { x: 5, y: 10 };
    println!("p.x={}", p.x());
    //
    let f32_point = Point { x: 5.0, y: 10.0 };
    println!(
        "Declared only for f32 points: {:?} distance is: {}",
        f32_point,
        f32_point.distance_from_origin()
    );
    //
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    println!("Mixup types of points: 1:{:?}, 2:{:?}", p1, p2);
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    //
    println!("Compare simple vector by generic function");
    let number_list = vec![34, 50, 25, 100, 65];
    println!("Check array: {:?}", number_list);
    let max_number = max(&number_list).expect("Error");
    println!("The largest number is {}", max_number);
    //
    let zero_sized_arr: Vec<i32> = Vec::new();
    println!("Try check array: {:?}", zero_sized_arr);
    if let Some(x) = max(&zero_sized_arr) {
        println!("The largest number is {}", x);
    } else {
        println!("Array was empty");
    }
    //
    println!("Compare points by generic function:");

    let point_arr: Vec<Point<i32, i32>> = vec![
        Point { x: 2, y: 3 },
        Point { x: 2, y: 4 },
        Point { x: 0, y: 0 },
    ];
    println!("The largest point is {:?}", max(&point_arr).unwrap());
}
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//implement only for f32 points (specialization)
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
