#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self, value: i32) -> Option<i32> {
        let result = self.list.iter().position(|x| *x == value);
        match result {
            Some(index) => {
                let result = self.list.remove(index);
                self.update_average();
                Some(result)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
    fn new() -> AveragedCollection {
        return AveragedCollection {
            list: vec![],
            average: 0.0,
        };
    }
}
#[allow(dead_code)]
pub fn run() {
    println!("Example of class, that can cache average value:");
    let mut a = AveragedCollection::new();
    for i in 0..10 {
        a.add(i);
    }
    a.remove(0);
    println!("Average of {:?} is {}", a, a.average())
}
