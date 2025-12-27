fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let y = 10;
    let y = y + 2;
    {
        let y = y * 3;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let name = "Alice";
    let name_size = name.len();

    println!("The length of the name '{}' is: {}", name, name_size);

    let mut city = "Dublin";
    let city_size = city.len();
    println!("The length of the city '{}' is: {}", city, city_size);
    city = "Cork";
    println!("The city has been changed to '{}'", city);
}
