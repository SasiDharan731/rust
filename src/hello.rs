pub fn new() {
    let mut name = String::from("Hi sasi");
    name.push_str("How are you?");
    
    let names: (i8, i8, &str) = (20, 3, "Sasi");
    
   print!("{}", names.2);
}