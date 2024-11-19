use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    for item in vec!["89", "8", "9.0", "eleven", "6060"] {
        let parsed = item.parse::<i32>()?;
        println!("{parsed}")
    }
    Ok(())
}
