fn main() {
    let alphabet = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    println!("{:?}", alphabet)
}
