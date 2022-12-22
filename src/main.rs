fn main() {
    // Create an array of integers from 0 to 9
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Array length {}", numbers.len());
    // Loop through the array
    for number in numbers.iter() {
        // Print each number
        println!("{}", number);
    }
}
