// function for retrieving upper and lowercase characters
pub fn get_alphabet() -> Vec<char> {
    let alphabet = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    return alphabet;
}
