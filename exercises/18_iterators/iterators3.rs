#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

// Function to calculate a / b
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }
    
    Ok(a / b)
}

// Function to return the results in a list
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));  // No need to dereference `n`
    
    division_results.collect()  // Collect the results into a vector
}

// Function to return the list of results in Result form
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));  // No need to dereference `n`
    
    division_results.collect()  // Collect the results as a Vec<Result>
}

fn main() {
    // Optionally, you can experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
