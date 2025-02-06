use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// Here we modify the return type to Result<(), Box<dyn Error>>
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    
    // Parse the user input into a number
    let x: i64 = pretend_user_input.parse()?;
    
    // Call the function, which could return an error
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    
    // Return Ok if everything went smoothly
    Ok(())
}
