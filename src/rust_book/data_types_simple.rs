//on overflow happens panic
//integer
#[allow(dead_code)]
pub fn run() {
    let _int: i8 = 0;
    let _int: u8 = 0;
    let _int: i16 = 0;
    let _int: u16 = 0;
    let _int: i32 = 0;
    let _int: u32 = 0;
    let _int: i128 = 0;
    let _int: u128 = 0;
    //architecture dependent
    let _arch: isize = 0;
    let _arch: usize = 0;
    //different representation
    let _byte: u8 = b'A'; //u8 only
    let _hex: u8 = 0xFF;
    let _oct: u8 = 0o77;
    let _bin: u8 = 0b1111_0000;
    //floating point
    let _fp: f64 = 3.0; //prefer over f32
    let _fp: f32 = 3.0;
    //boolean
    let _t: bool = true;
    let _t: bool = false;
    //character
    let _c: char = 'z'; //4 bytes unicode

    //compound Types
    //tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; //pattern matching
    println!("The value of y is:{}", y);
    //array (allocated on the stack rather than the heap)
    let _arr = [1, 2, 3, 4, 5]; //always fixed length
    let _arr: [i32; 3] = [1, 2, 3];
    let arr = [3; 5]; //3 (5 times)
    let _first = arr[0];
    let _second = arr[1];
}
