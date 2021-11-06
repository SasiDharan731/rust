pub fn new() {
    let b = [10, 20, 30, 5, 3, 9, 10, 50, 88];
    for nums in b{
        if nums % 2 == 0 {
            println!("The number {} is even", nums)
        }else{
            println!("The number {} is odd", nums)
        }

    }
}