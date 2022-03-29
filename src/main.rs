use std::io;
mod alphabet;

fn main() {
    println!("Welcome to Zip-cracker!");

    let mut password = String::new();

    // characters vector of characters to mutate with
    let characters = vec!['a'];

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read input");

    // remove final \n
    password.pop();

    let password_chars: Vec<char> = password.chars().collect();

    println!("{:?}", password_chars)
    //    for (i,c) in password.chars().enumerate() {
    //
    //    }
}
