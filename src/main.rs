fn main() {
    // Create an array of integers from 0 to 9
    // let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("Array length {}", numbers.len());
    // Loop through the array
    // for number in numbers.iter() {
    //     // Print each number
    //     println!("{}", number);
    // }
    // println!("Print 1-9");

    // for i in 0..10 {
    //     // Print each number
    //     println!("{}", i);
    // }
    // println!("Loop through the characters in a string");

    // let s = "hello";

    // // Loop through the characters in the string
    // for c in s.chars() {
    //     // Print each character
    //     println!("{}", c);
    // }
    // let array = [10, 20, 30];
    // println!("array: {array:?}");
    // let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    // pretty_print(&matrix);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let transposed = transpose(&matrix);
    pretty_print(&transposed);
}
fn pretty_print(matrix: &[[i32; 3]; 3]) {
    // Loop through the rows of the matrix
    for row in matrix {
        // Loop through the elements in the row
        for element in row {
            // Print each element with a space between them
            print!("{} ", element);
        }
        // Print a newline after each row
        println!();
    }
}
fn transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    // Loop through the rows and columns of the matrix
    for i in 0..3 {
        for j in 0..3 {
            // Transpose the matrix by swapping the rows and columns
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}
