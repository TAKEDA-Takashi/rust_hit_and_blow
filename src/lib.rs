pub mod numbers;
pub mod validation;

pub fn input_read_line() -> std::io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
