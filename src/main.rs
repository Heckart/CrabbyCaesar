use std::io;

use crate::crabby_caesar::caesar_translate;
mod crabby_caesar;

fn main() {
    // Purpose:    IO, and calls your functions.
    // Parameters: None
    // User Input: Input text to translate
    // Prints:     Print result
    // Returns:    Nothing
    // Modifies:   Nothing outside its scope
    // Calls:      std::
    // Tests:      None
    // Status:     Do this one.

    let mut message: String = String::new();
    println!("Enter a string to translate:");
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    message = message.trim_end_matches('\n').to_string(); // message.trim() can remove important spaces that need to be kept

    let mut mode: String = String::new();
    println!("encrypt or decrypt?");
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");
    let mode = mode.trim();

    let mut key_str: String = String::new();
    println!("What is your key?");
    io::stdin()
        .read_line(&mut key_str)
        .expect("Failed to read line");
    let key: isize = key_str.trim().parse().expect("Input not an integer"); // https://doc.rust-lang.org/std/string/struct.String.html#method.trim

    let translated: String = caesar_translate(message.to_string(), mode.to_string(), key);

    println!("{}\n", translated);
}
