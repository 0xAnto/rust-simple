fn main() {
    // Create an array of integers from 0 to 9
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Array length {}", numbers.len());
    // Loop through the array
    for number in numbers.iter() {
        // Print each number
        println!("{}", number);
    }
    println!("Print 1-9");

    for i in 0..10 {
        // Print each number
        println!("{}", i);
    }
    println!("Loop through the characters in a string");

    let s = "hello";

    // Loop through the characters in the string
    for c in s.chars() {
        // Print each character
        println!("{}", c);
    }
}
