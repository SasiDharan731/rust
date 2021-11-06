use std::io;
pub fn new() {
    let mut input = String::new();
    println!("Please enter a string value");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    println!("You have typed {}", input)

}