use itertools::Itertools;
fn main() {
    // Set the maximum length of the password
    let max_password_length = 8;

    // Create a list of all possible characters that could be in the password
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    // Iterate over all possible combinations of characters up to the maximum password length
    for length in 1..=max_password_length {
        for password_guess in characters.chars().combinations(length) {
            let password_guess: String = password_guess.iter().collect();
            // Attempt to login using the current password guess
            if login(&password_guess) {
                // If the login is successful, print the password and exit the program
                println!("Password found: {}", password_guess);
                return;
            }
        }
    }
    println!("Password not found");
}
