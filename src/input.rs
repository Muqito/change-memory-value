pub fn read_input<T>(prompt: impl std::fmt::Display) -> Option<T>
where
    T: std::str::FromStr,
{
    println!("{}", prompt);
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }

    input = input.trim().parse().unwrap();

    T::from_str(&input).ok()
}
pub fn read_memory_addr(prompt: impl std::fmt::Display) -> Option<usize> {
    println!("{}", prompt);
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }

    usize::from_str_radix(input.trim_start_matches("0x").trim_end(), 16).ok()
}
