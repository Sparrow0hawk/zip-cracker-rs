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

    println!("{:?}", password_chars);

    let mut passwords: Vec<String> = vec![];

    for (i, c) in password.chars().enumerate() {
        for letter in characters.iter() {
            // create a cloned vector of password characters
            let mut trial_pwd = password_chars.clone();

            // change character at ith index using letter var
            trial_pwd[i] = *letter;

            // create a new string from the trial_pwd vec with mutated char
            let trial_pwd_str: String = trial_pwd.into_iter().collect();

            // append this string to passwords Vec
            passwords.push(trial_pwd_str);
        }
    }
    println!("{:?}", passwords);
}
