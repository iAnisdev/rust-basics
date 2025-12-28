fn main() {
    // default integer types in Rust is i32
    //i8 , i16, i32, i64, i128 , isize
    //u8 , u16, u32, u64, u128 , usize
    // isize and usize are pointer-sized integers and depend on the architecture of the machine (32-bit or 64-bit)
    let a: i8 = 10; // 8-bit signed integer from -128 to 127
    let b: u8 = 212; // 8-bit unsigned integer from 0 to 255

    println!("a: {}, b: {}", a, b);
    println!("Min and Max values of i8: {} to {}", std::i8::MIN, std::i8::MAX);
    println!("Min and Max values of isize: {} to {}", std::isize::MIN, std::isize::MAX);
    println!("Min and Max values of usize: {} to {}", std::usize::MIN, std::usize::MAX);

    let trillion: u64 = 1_000_000_000_000; 
    println!("Trillion: {}", trillion);

    let pi: f32 = 3.14; // 32-bit floating point
    let e: f64 = 2.718281828459045; // 64-bit floating point
    println!("Pi: {}, e: {}", pi, e);


    let is_rust_fun: bool = true;
    let is_dublin_hot: bool = false;
    println!("Is Rust fun? {}, Is Dublin hot? {}", is_rust_fun, is_dublin_hot);

    let letter: char = 'R';
    println!("Letter: {}", letter);
    let name: &str = "Rust Programming";
    println!("Name: {}", name);

    let s = String::from("Hello, Rust!");
    println!("String: {}", s);
}
